extern crate tokio;

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};

    use tokio::prelude::*;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::{io as tokio_io};

    pub static LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

    #[test]
    fn client_server() {
        let listener = TcpListener::bind(&SocketAddr::new(LOCALHOST, 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = listener.incoming()
        .map_err(|e| panic!(e))
        .take(1)
        .for_each(|sock| {
            let (reader, writer) = sock.split();
            tokio::spawn(
                tokio_io::copy(reader, writer)
                .map(|_| ())
                .map_err(|e| panic!(e))
            )
        });

        let client = TcpStream::connect(&address)
        .and_then(|stream| {
            tokio_io::write_all(stream, b"hello, server\n")
        })
        .and_then(|(stream, text)| {
            let buf = vec!(0; text.len());
            tokio_io::read_exact(stream, buf)
            .map(move |(stream, buf)| (stream, text, buf))
        })
        .map(|(_, text, buf)| {
            assert_eq!(text, &buf[..]);
        })
        .map_err(|e| panic!(e));

        tokio::run(server.join(client).map(|_| ()));
    }
}
