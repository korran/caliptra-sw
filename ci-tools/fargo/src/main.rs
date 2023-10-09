
use std::io::Read;
use std::{path::{PathBuf, Path}, net::{SocketAddr, IpAddr}, str::FromStr, fmt::Display, process::{Command, Stdio}, io};
use caliptra_builder::firmware;
use cargo_metadata::Message;

struct RemoteHost {
    hostname: String,
    port: u16,
}
impl Display for RemoteHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hostname, self.port)
    }
}
impl FromStr for RemoteHost {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        const DEFAULT_PORT: u16 = 22;

        let format_ip = |ip| match ip {
            IpAddr::V4(addr) => addr.to_string(),
            IpAddr::V6(addr) => format!("[{addr}])"),
        };

        if let Ok(socket_addr) = SocketAddr::from_str(value) {
            return Ok(RemoteHost{
                hostname: format_ip(socket_addr.ip()),
                port: socket_addr.port(),
            })
        }
        if let Ok(ip) = IpAddr::from_str(value) {
            return Ok(RemoteHost{
                hostname: format_ip(ip),
                port: DEFAULT_PORT,
            })
        }

        if let Some((hostname, port)) = value.split_once(':') {
            return Ok(RemoteHost{
                hostname: hostname.into(),
                port: u16::from_str(port).map_err(|_| ())?,
            });
        }
        Ok(RemoteHost{
            hostname: value.into(),
            port: DEFAULT_PORT,
        })
    }
}

pub fn run_cmd_stdout(cmd: &mut Command, input: Option<&[u8]>) -> io::Result<Vec<u8>> {
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    if let (Some(mut stdin), Some(input)) = (child.stdin.take(), input) {
        std::io::Write::write_all(&mut stdin, input)?;
    }
    let out = child.wait_with_output()?;
    if out.status.success() {
        Ok(out.stdout)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Process {:?} {:?} exited with status code {:?} stdout {} stderr {}",
                cmd.get_program(),
                cmd.get_args().collect::<Vec<_>>(),
                out.status.code(),
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            ),
        ))
    }
}



pub fn run_cmd(cmd: &mut Command) -> io::Result<()> {
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Process {:?} {:?} exited with status code {:?}",
                cmd.get_program(),
                cmd.get_args().collect::<Vec<_>>(),
                status.code()
            ),
        ))
    }
}

fn ssh_cmd(host: &RemoteHost, command_str: &str) -> io::Result<()> {
    let mut cmd = Command::new("ssh");
    cmd.arg(&host.hostname);
    cmd.arg("-l");
    cmd.arg("root");
    cmd.arg("-p");
    cmd.arg(host.port.to_string());
    cmd.arg(command_str);
    run_cmd(&mut cmd)
}

fn scp(files: &[PathBuf], host: &RemoteHost, dest: &Path) -> io::Result<()> {
    if files.is_empty() {
        return Ok(())
    }

    let mut cmd = Command::new("rsync");
    cmd.arg("-e");
    cmd.arg(format!("ssh -p {}", host.port));
    for file in files {
        cmd.arg(file);
    }
    cmd.arg(format!("root@{}:{}", host.hostname, dest.display()));
    run_cmd(&mut cmd)
}

enum ParseMode {
    WaitingForTest,
    WaitingForTestName,
    HasTestName,
    TestArgs,
}


fn main() {
    let Ok(Ok(remote_host)) = std::env::var("FARGO_REMOTE_HOST").map(|s| RemoteHost::from_str(s.as_str())) else {
        panic!("Expected FARGO_REMOTE_HOST environment variable was not set or was invalid. Examples: 'myhost', 'myhost:4022', '192.168.1.2:22', '[2001:0db8::1]:8123'");
    };

    let Ok(Ok(sysroot)) = std::env::var("FARGO_SYSROOT").map(|s| RemoteHost::from_str(s.as_str())) else {
        panic!("Expected FARGO_SYSROOT environment variable was not set or was invalid.");
    };

    let Ok(Ok(bitstream)) = std::env::var("FARGO_BITSTREAM").map(|s| RemoteHost::from_str(s.as_str())) else {
        panic!("Expected FARGO_BITSTREAM environment variable was not set or was invalid.");
    };

    let mut cargo_inv = Command::new("cargo");
    cargo_inv.arg("--config");
    cargo_inv.arg(format!(
        "target.aarch64-unknown-linux-gnu.rustflags = [\"-C\", \"link-arg=--sysroot={}\"]", sysroot));

    cargo_inv.arg("--config");
    cargo_inv.arg("target.aarch64-unknown-linux-gnu.linker = \"aarch64-linux-gnu-gcc\"");

    let mut mode = ParseMode::WaitingForTest;
    let mut test_args = vec![];
    for arg in std::env::args().skip(1) {
        match mode {
            ParseMode::WaitingForTest if arg == "test" => {
                cargo_inv.arg("--target=aarch64-unknown-linux-gnu");
                cargo_inv.arg("--no-run");
                cargo_inv.arg("--message-format=json");
                mode = ParseMode::WaitingForTestName;
            }
            ParseMode::WaitingForTestName if !arg.starts_with('-') => {
                test_args.push(arg.clone());
                mode = ParseMode::HasTestName;
            }
            ParseMode::TestArgs => {
                test_args.push(arg.clone());
            }
            _ => {}
        }
        if arg == "--" {
            mode = ParseMode::TestArgs;
        }
    }

    let mut test_files = vec![];
    let json = run_cmd_stdout(&mut cargo_inv, None).unwrap();
    for msg in cargo_metadata::Message::parse_stream(json.as_slice()) {
        let Message::CompilerArtifact(artifact) = msg.unwrap() else {
            continue;
        };
        let Some(executable) = &artifact.executable else {
            continue;
        };
        if !artifact.target.test {
            continue;
        }
        test_files.push(executable.clone().into_std_path_buf());
        println!("{:?}", artifact);

    }
    ssh_cmd(&remote_host, "mkdir -p /tmp/fargo-tests").unwrap();
    scp(&test_files, &remote_host, Path::new("/tmp/fargo-tests")).unwrap();

    let home_dir = PathBuf::from(std::env::var("HOME").unwrap());
    let fargo_dir = home_dir.join(".caliptra-fargo");

    let fw_dir = Path::new("/tmp/fargo-fw");
    std::fs::create_dir_all(fw_dir).unwrap();
    let mut fw_files = vec![];
    for (fwid, elf_bytes) in
        caliptra_builder::build_firmware_elfs_fast(firmware::REGISTERED_FW).unwrap()
    {
        let path = fw_dir.join(fwid.elf_filename());
        std::fs::write(&path, elf_bytes).unwrap();
        fw_files.push(path);
    }
    ssh_cmd(&remote_host, "mkdir -p /tmp/fargo-fw").unwrap();
    scp(&fw_files, &remote_host, fw_dir).unwrap();

    // TODO: Escape?
    let test_args_str = test_args.join(" ");
    for test_file in test_files {
        let test_name = test_file.file_name().unwrap().to_str().unwrap();
        ssh_cmd(&remote_host, 
            &format!("bash -c \"CALIPTRA_PREBUILT_FW_DIR=/tmp/fargo-fw CALIPTRA_IMAGE_NO_GIT_REVISION=1 /tmp/fargo-tests/{test_name}\" {test_args_str}")).unwrap();
    }
}
