use std::io;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn tcp_connect(target: SocketAddr, timeout: Duration) -> io::Result<()> {
    let result = TcpStream::connect_timeout(&target, timeout);
    result.map(|_| ())
}
