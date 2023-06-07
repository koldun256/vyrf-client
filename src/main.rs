use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8888")?;
    let server = "127.0.0.1:8080";

    socket.send_to(&[0x04, 0x05, 0x06], server)?;

    let mut buffer = [0; 1024];
    let (len, _) = socket.recv_from(&mut buffer)?;
    let result = &buffer[..len];
    for byte in result {
        print!("{} ", byte);
    }
    println!();
    Ok(())
}