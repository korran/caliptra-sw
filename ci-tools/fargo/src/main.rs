
use std::{path::PathBuf, net::{SocketAddr, IpAddr}, str::FromStr, fmt::Display};
use caliptra_builder::firmware;

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

fn main() {
    let Ok(Ok(remote_host)) = std::env::var("FARGO_REMOTE_HOST").map(|s| RemoteHost::from_str(s.as_str())) else {
        panic!("Expected FARGO_REMOTE_HOST environment variable was not set or was invalid. Examples: 'myhost', 'myhost:4022', '192.168.1.2:22', '[2001:0db8::1]:8123'");
    };
    let home_dir = PathBuf::from(std::env::var("HOME").unwrap());
    let fargo_dir = home_dir.join(".caliptra-fargo");
    for (fwid, elf_bytes) in
        caliptra_builder::build_firmware_elfs_fast(firmware::REGISTERED_FW).unwrap()
    {
    }
}
