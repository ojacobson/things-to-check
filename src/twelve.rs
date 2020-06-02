//! A [twelve-factor application][1] reads [its configuration][2] from the environment.
//! 
//! In many cases, "read" directly maps to the target binary inspecting the
//! OS-provided environment dictionary. This module provides supporting tools
//! for reading configuration data from the environment, via `std::env`, and
//! converting it to useful types.
//! 
//! [1]: https://12factor.net/
//! [2]: https://12factor.net/config

use std::env;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, IpAddr, SocketAddr, ToSocketAddrs};
use std::num;
use thiserror::Error;

/// Errors that can arise when reading a port number from the environment.
/// 
/// For convenience when returning errors into `main`, this type can be
/// converted to std::io::Error.
#[derive(Error, Debug)]
pub enum Error {
    /// PORT was set, but contained a non-unicode value that sys::env can't parse.
    /// 
    /// For obvious reasons, this cannot be converted to a port number. Rather
    /// than ignoring this error, we report it, so that misconfiguration can be
    /// detected early.
    #[error("PORT must be a number ({source})")]
    NotUnicode {
        #[from]
        source: env::VarError,
    },
    /// PORT was set, but was set to a non-numeric value.FnOnce
    /// 
    /// PORT can only be used to select a port number if numeric. Rather than
    /// ignoring this error, we report it, so that misconfiguration can be
    /// detected early.
    #[error("PORT must be a number ({source})")]
    ParseError {
        #[from]
        source: num::ParseIntError,
    }
}

/// A listen address consisting of only a port number.
/// 
/// Listening on this address will bind to both the ip4 and ip6 addresses on the
/// current host, assuming both ip4 and ip6 are supported.
#[derive(Debug, Clone)]
pub struct PortAddr {
    /// When used in an std::net::SocketAddr context, this is the port number to
    /// bind on.
    port: u16
}

fn v4(port_addr: &PortAddr) -> SocketAddr {
    SocketAddr::new(IpAddr::from(Ipv4Addr::UNSPECIFIED), port_addr.port)
}

fn v6(port_addr: &PortAddr) -> SocketAddr {
    SocketAddr::new(IpAddr::from(Ipv6Addr::UNSPECIFIED), port_addr.port)
}

impl ToSocketAddrs for PortAddr {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        let addrs = vec![
            v6(self),
            v4(self),
        ];

        Ok(addrs.into_iter())
    }
}

/// Query the environment for a port number.
/// 
/// This will read the PORT environment variable. If set, it will use the value
/// (as a number). If it's unset, then this will use the passed `default_port`
/// number to choose the app's default port. If the PORT environment variable
/// is set but cannot be interpreted as a port number, this will return an error
/// indicating why, to assist the user in correcting their configuration.
/// # Examples
/// 
/// ```
/// use std::net::TcpListener;
/// mod twelve;
/// 
/// // Listen on port 3000 (or $PORT if set), on global ip4 and ip6 interfaces.
/// let port = twelve::port(3000)?;
/// let listener = TcpListener::bind(port);
/// ```
pub fn port(default_port: u16) -> Result<PortAddr, Error> {
    let port = match env::var("PORT") {
        Ok(env_port) => env_port.parse()?,
        Err(e) => match e {
            env::VarError::NotPresent => default_port,
            env::VarError::NotUnicode(_) => return Err(Error::from(e)),
        },
    };

    Ok(PortAddr{
        port,
    })
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use quickcheck::{Arbitrary, Gen, TestResult};
    use quickcheck_macros::quickcheck;
    use std::env;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::sync::Mutex;

    use super::*;

    impl Arbitrary for PortAddr {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Self {
                port: u16::arbitrary(g),
            }
        }
    }

    #[quickcheck]
    fn port_addr_as_socket_addr_has_v4(addr: PortAddr) -> bool {
        let socket_addrs = addr.to_socket_addrs()
            .unwrap()
            .collect::<Vec<_>>();

        socket_addrs.iter()
            .any(|&socket_addr| socket_addr.is_ipv4())
    }

    #[quickcheck]
    fn port_addr_as_socket_addr_has_v6(addr: PortAddr) -> bool {
        let socket_addrs = addr.to_socket_addrs()
            .unwrap()
            .collect::<Vec<_>>();

        socket_addrs.iter()
            .any(|&socket_addr| socket_addr.is_ipv6())
    }

    #[quickcheck]
    fn port_addr_as_socket_addr_all_have_port(addr: PortAddr) -> bool {
        let socket_addrs = addr.to_socket_addrs()
            .unwrap()
            .collect::<Vec<_>>();

        socket_addrs.iter()
            .all(|&socket_addr| socket_addr.port() == addr.port)
    }

    #[derive(Default)]
    struct Runner;

    impl Runner {
        // This mostly serves to keep a mutex locked for the duration of a
        // function. See ENV_MUTEX, below.
        fn run<T>(&self, f: impl FnOnce() -> T) -> T {
            f()
        }
    }

    lazy_static! {
        // The tests in this module manipulate a global, shared, external
        // resource (the PORT environment variable). The quickcheck tool
        // attempts to accelerate testing by running multiple threads, but this
        // causes race conditions as test A stomps on state used by test B.
        // Serialize tests through a mutex.
        // 
        // Huge hack.
        static ref ENV_MUTEX: Mutex<Runner> = Mutex::new(Runner::default());
    }

    // Runs a body with ENV_MUTEX locked. Easier to write.
    fn env_locked<T>(f: impl FnOnce() -> T) -> T {
        ENV_MUTEX.lock()
            .unwrap()
            .run(f)
    }

    #[quickcheck]
    fn port_preserves_numeric_values(env_port: u16, default_port: u16) -> TestResult {
        if env_port == default_port {
            return TestResult::discard();
        }

        env_locked(|| {
            env::set_var("PORT", env_port.to_string());

            let read_port = port(default_port)
                .unwrap();

            TestResult::from_bool(read_port.port == env_port)
        })
    }

    #[quickcheck]
    fn port_rejects_strings(env_port: String, default_port: u16) -> TestResult {
        if env_port.contains("\x00") {
            return TestResult::discard();
        }

        env_locked(|| {
            env::set_var("PORT", env_port.to_string());

            let port_result = port(default_port);

            TestResult::from_bool(port_result.is_err())
        })
    }

    #[quickcheck]
    fn port_uses_default(default_port: u16) -> bool {
        env_locked(|| {
            env::remove_var("PORT");

            let read_port = port(default_port)
                .unwrap();

            read_port.port == default_port
        })
    }

    #[test]
    fn port_non_unicode() {
        let non_unicode = OsStr::from_bytes(&[0xF5u8]);

        env_locked(|| {
            env::set_var("PORT", non_unicode);

            let result = port(1234);

            assert!(result.is_err());
        })
    }
}