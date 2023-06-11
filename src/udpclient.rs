use std::sync::mpsc::Sender;
use std::net::UdpSocket;
use std::io::stdin;

pub enum ServerMsg {
    Val(i32)
}

pub enum ClientMsg {
    Increment
}

pub fn open_udp_socket() -> UdpSocket {
    let mut port = String::new();
    stdin().read_line(&mut port).expect("error reading port from cli");
    port.pop();
    UdpSocket::bind(format!("127.0.0.1:{port}")).expect("cannot open udp socket")
}

pub fn start_udp_thread(tx: Sender<ServerMsg>, socket: &UdpSocket) {
    loop {
        let mut answer = [0; 4];
        socket.recv_from(&mut answer).expect("error reading msg from server");
        tx.send(ServerMsg::Val(i32::from_be_bytes(answer)));
    }
}

pub fn send_udp_msg(msg: ClientMsg, socket: &UdpSocket) {
    let server = "127.0.0.1:8080";
    match msg {
        ClientMsg::Increment => socket.send_to(&[1], server)
    };
}