use std::sync::mpsc::{Receiver, Sender, channel};
use std::net::UdpSocket;
use std::thread;

pub enum ServerMsg {
    Val(i32)
}

pub enum ClientMsg {
    Increment
}

fn recv_msg(buf: &[u8; 4]) -> ServerMsg {
    ServerMsg::Val(i32::from_be_bytes(*buf))
}

fn send_msg(msg: ClientMsg, socket: &UdpSocket) {
    let server = "127.0.0.1:8080";
    match msg {
        ClientMsg::Increment => socket.send_to(&[1], server)
    };
}

pub fn connect(port: &str) -> (Sender<ClientMsg>, Receiver<ServerMsg>) {
    let (tx1, rx1) = channel(); // from client to server
    let (tx2, rx2) = channel(); // from server to client
    let socket1 = UdpSocket::bind(format!("127.0.0.1:{port}")).expect("cannot open udp socket");
    let socket2 = socket1.try_clone().unwrap();
    thread::spawn(move || loop {
        send_msg(rx1.recv().unwrap(), &socket1);
    });
    thread::spawn(move || loop {
        let mut answer = [0; 4];
        socket2.recv_from(&mut answer).expect("error reading msg from server");
        tx2.send(recv_msg(&answer));
    });
    (tx1, rx2)
}