use std::sync::mpsc::{Receiver, Sender, channel};
use std::net::UdpSocket;
use std::thread;

enum Error {
    InvalidMsg
}
pub enum ServerMsg { 
    AddObject { id: u8, kind: u8, x: u16, y: u16 },
    BindPlayer { id: u8 }
}

pub enum ClientMsg {
    Register
}

fn parse_msg(buf: &[u8; 10]) -> Result<ServerMsg, Error> {
    println!("{:?}", buf);
    if buf[0] == 0 {
        Ok(ServerMsg::AddObject { 
            id: buf[1],
            kind: buf[2],
            x: buf[3] as u16 * 256 + buf[4] as u16,
            y: buf[5] as u16 * 256 + buf[6] as u16
        })
    } else if buf[0] == 1 {
        Ok(ServerMsg::BindPlayer { id: buf[1] })
    } else {
        Err(Error::InvalidMsg)
    }
}

fn gen_payload(msg: ClientMsg) -> [u8; 10] {
    match msg {
        ClientMsg::Register => [0; 10]
    }
}

pub fn connect(port: &str) -> (Sender<ClientMsg>, Receiver<ServerMsg>) {
    let (tx1, rx1) = channel(); // from client to server
    let (tx2, rx2) = channel(); // from server to client
    let socket1 = UdpSocket::bind(format!("127.0.0.1:{port}")).expect("cannot open udp socket");
    let socket2 = socket1.try_clone().unwrap();

    thread::spawn(move || while let Ok(msg) = rx1.recv() {
        socket1.send_to(&gen_payload(msg), "127.0.0.1:8080");
    });

    thread::spawn(move || {
        let mut answer = [0; 10];
        while let Ok(_) = socket2.recv_from(&mut answer) {
            match parse_msg(&answer) {
                Ok(msg) => tx2.send(msg).unwrap(),
                Err(Error::InvalidMsg) => println!("invalid server response {:?}", answer)
            }
        }
    });
    (tx1, rx2)
}