mod find_usb_block_device;
mod fpga_jtag;
mod ftdi;
mod ftdi_uart;
mod sd_mux;
mod tty;
mod usb_port_path;

use anyhow::Context;
use find_usb_block_device::find_usb_block_device;

use clap::{arg, value_parser};
use libftdi1_sys::ftdi_interface;
use std::{
    fs::{File, OpenOptions},
    io::{stdout, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

pub(crate) use fpga_jtag::{FpgaJtag, FpgaReset};
pub(crate) use ftdi::FtdiCtx;
pub(crate) use ftdi_uart::FtdiUart;
pub(crate) use sd_mux::{SdMux, SdMuxTarget};
pub(crate) use usb_port_path::UsbPortPath;

fn cli() -> clap::Command<'static> {
    clap::Command::new("caliptra-emu")
        .about("Caliptra emulator")
        .arg(
            arg!(--"sdwire" <PORT_PATH> "USB port path to the hub chip on the SDWire (ex: 3-1.2)")
                .value_parser(value_parser!(UsbPortPath))
        )
        .arg(
            arg!(--"zcu104" <PORT_PATH> "USB port path to the FTDI chip on the ZCU104 dev board (ex: 3-1.2)")
                .value_parser(value_parser!(UsbPortPath))
        )
        .subcommand_required(true)
        .subcommand(clap::Command::new("mode")
            .about("Set the state of the reset / sdwire pins")
            .arg(arg!(<MODE>).value_parser(value_parser!(SdMuxTarget))))
            .arg_required_else_help(true)
        .subcommand(clap::Command::new("flash")
            .about("Flash an image file to the sdwire and boot the DUT")
            .arg(arg!(-r --read_first).takes_value(false).help("To reduce write cycles, read contents first, and don't flash if they are already as expected."))
            .arg(arg!(<IMAGE_FILENAME>).value_parser(value_parser!(PathBuf))))
        .subcommand(clap::Command::new("console")
            .about("Tail the UART output from zcu104"))
}

fn main() {
    match main_impl() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("Fatal error: {e:#}");
            std::process::exit(1);
        }
    }
}

fn open_block_dev(path: &Path) -> std::io::Result<File> {
    let mut tries = 0_u32;
    loop {
        match OpenOptions::new().read(true).write(true).open(path) {
            Ok(result) => return Ok(result),
            Err(err) => {
                if err.raw_os_error() == Some(libc::ENOMEDIUM as i32) {
                    if tries == 0 {
                        println!("Waiting for attached sd card to be noticed by OS")
                    }
                    // SD card hasn't been found by the OS yet
                    tries += 1;
                    if tries < 1000 {
                        std::thread::sleep(Duration::from_millis(10));
                        continue;
                    }
                }
            }
        }
    }
}

/// Returns true if the device alread contains the image.
fn verify_image(dev: &mut File, image: &mut File) -> std::io::Result<bool> {
    dev.seek(SeekFrom::Start(0))?;
    let file_len = image.metadata()?.len();
    let mut buf1 = vec![0_u8; 1024 * 1024];
    let mut buf2 = vec![0_u8; 1024 * 1024];
    let mut total_read: u64 = 0;
    let start_time = Instant::now();
    loop {
        let bytes_read = image.read(&mut buf1)?;
        if bytes_read == 0 {
            return Ok(true);
        }
        dev.read_exact(&mut buf2[..bytes_read])?;
        if buf1[..bytes_read] != buf2[..bytes_read] {
            return Ok(false);
        }
        total_read += u64::try_from(bytes_read).unwrap();
        let duration = Instant::now() - start_time;
        print!(
            "Read {} MB of {} MB: {:.1} MB/sec \r",
            total_read / (1024 * 1024),
            file_len / (1024 * 1024),
            total_read as f64 / duration.as_secs_f64() / (1024.0 * 1024.0)
        );
        std::io::stdout().flush()?;
    }
}

fn copy_file(dest: &mut File, src: &mut File) -> std::io::Result<()> {
    src.seek(SeekFrom::Start(0))?;
    dest.seek(SeekFrom::Start(0))?;
    let file_len = src.metadata()?.len();
    let mut buf = vec![0_u8; 1024 * 1024];
    let mut total_written: u64 = 0;
    let start_time = Instant::now();
    loop {
        let bytes_read = src.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        total_written += u64::try_from(bytes_read).unwrap();
        dest.write_all(&buf[..bytes_read])?;
        dest.sync_data()?;
        let duration = Instant::now() - start_time;
        print!(
            "Wrote {} MB of {} MB: {:.1} MB/sec \r",
            total_written / (1024 * 1024),
            file_len / (1024 * 1024),
            total_written as f64 / duration.as_secs_f64() / (1024.0 * 1024.0)
        );
        std::io::stdout().flush()?;
    }
    Ok(())
}

fn main_impl() -> anyhow::Result<()> {
    let matches = cli().get_matches();
    let sdwire_hub_path = matches.get_one::<UsbPortPath>("sdwire").unwrap();
    let zcu104_path = matches.get_one::<UsbPortPath>("zcu104").unwrap();

    let mut sd_mux = SdMux::open(sdwire_hub_path.child(2))?;
    let mut fpga = FpgaJtag::open(zcu104_path.clone())?;
    let sd_dev_path = find_usb_block_device(&sdwire_hub_path.child(1)).with_context(|| {
        format!(
            "Could not find block device associated with {}",
            sdwire_hub_path.child(1)
        )
    })?;

    println!("SDWire block device is {}", sd_dev_path.display());

    match matches.subcommand() {
        Some(("mode", sub_matches)) => match sub_matches.get_one::<SdMuxTarget>("MODE").unwrap() {
            SdMuxTarget::Dut => {
                fpga.set_reset(FpgaReset::Reset)?;
                sd_mux.set_target(SdMuxTarget::Dut)?;
                std::thread::sleep(Duration::from_millis(1));
                fpga.set_reset(FpgaReset::Run)?;
            }
            SdMuxTarget::Host => {
                fpga.set_reset(FpgaReset::Reset)?;
                sd_mux.set_target(SdMuxTarget::Host)?;
            }
        },
        Some(("console", _)) => {
            let mut uart = FtdiUart::open(zcu104_path.clone(), ftdi_interface::INTERFACE_B)?;
            let mut stdout_buf = [0_u8; 4096];
            let mut stdin_buf = [0_u8; 4096];

            println!("To exit terminal type Ctrl-T then Q");
            // Will reset terminal back to regular settings upon drop
            let raw_terminal = tty::RawTerminal::from_stdin()?;

            let mut escaper = tty::EscapeSeqStateMachine::new();
            let mut alive = true;

            while alive {
                let mut stdin_len = tty::read_stdin_nonblock(&mut stdin_buf)?;
                escaper.process(&mut stdin_buf, &mut stdin_len, |ch| match ch {
                    b'q' | b'Q' => alive = false,
                    ch => println!(
                        "Unknown escape sequence: Ctrl-T + {:?}",
                        char::from_u32(ch.into()).unwrap_or('?')
                    ),
                });
                if stdin_len > 0 {
                    uart.write_all(&stdin_buf[..stdin_len])?;
                }
                let stdout_len = uart.read(&mut stdout_buf)?;
                if stdout_len > 0 {
                    stdout().write_all(&stdout_buf[..stdout_len])?;
                    stdout().flush()?;
                }
            }
            drop(raw_terminal);
            println!("");
        }
        Some(("flash", sub_matches)) => {
            fpga.set_reset(FpgaReset::Reset)?;
            sd_mux.set_target(SdMuxTarget::Host)?;
            let image_filename = sub_matches.get_one::<PathBuf>("IMAGE_FILENAME").unwrap();
            println!(
                "Flashing {} to {}",
                image_filename.display(),
                sd_dev_path.display()
            );
            let mut file =
                File::open(image_filename).with_context(|| image_filename.display().to_string())?;
            let mut sd_dev =
                open_block_dev(&sd_dev_path).with_context(|| sd_dev_path.display().to_string())?;
            if !sub_matches.is_present("read_first") || !verify_image(&mut sd_dev, &mut file)? {
                copy_file(&mut sd_dev, &mut file)?;
            } else {
                println!("Device already contains the desired image");
            }
            sd_mux.set_target(SdMuxTarget::Dut)?;
            std::thread::sleep(Duration::from_millis(100));
            fpga.set_reset(FpgaReset::Run)?
        }
        _ => unreachable!(),
    }
    Ok(())
}
