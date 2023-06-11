use std::net::UdpSocket;
use std::io::stdin;

fn main() -> std::io::Result<()> {
    let mut port = String::new();
    stdin().read_line(&mut port)?;
    port.pop();
    let socket = UdpSocket::bind(format!("127.0.0.1:{port}"))?;
    let server = "127.0.0.1:8080";

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input.pop();
        let increment: u8 = match input.parse() {
            Ok(i) => i,
            Err(_) => {
                println!("enter int from 0 to 255");
                continue;
            }
        };

        socket.send_to(&[increment], server).expect("cannot send msg");
        
        let mut answer = [0; 4];
        socket.recv_from(&mut answer)?;
        println!("{}", i32::from_be_bytes(answer));
    }
}