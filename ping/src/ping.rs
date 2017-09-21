use ::std::net;
use ::std::time;
use ::std::io;

use socket2::{SockAddr, Socket, Domain, Type};

pub type FnType = fn(Domain, &net::SocketAddr, time::Duration) -> Result<(bool, time::Duration), String>;

///Performs TCP connection and returns tuple (is_success, duration)
pub fn tcp(family: Domain, dest: &net::SocketAddr, timeout: time::Duration) -> Result<(bool, time::Duration), String> {
    let ty = Type::stream();
    let socket = match Socket::new(family, ty, None) {
        Ok(socket) => socket,
        Err(error) => return Err(format!("{}", error))
    };
    let dest = SockAddr::from(*dest);

    let now = time::Instant::now();
    match socket.connect_timeout(&dest, timeout) {
        Ok(_) => Ok((true, now.elapsed())),
        Err(error) => {
            match error.kind() {
                io::ErrorKind::TimedOut => Ok((false, now.elapsed())),
                _ => Err(format!("{}", error))
            }
        }
    }
}
