use std::env;
use sfml::window::Event;
use udp_client::{ServerMsg, ClientMsg};
use view::View;

mod udp_client;
mod view;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (udp_tx, udp_rx) = udp_client::connect(&args[1]);

    let resources = View::load_resources();
    let mut view = View::init(&resources);

    udp_tx.send(ClientMsg::Register).expect("udp thread is dead");

    while view.window.is_open() {
        while let Some(msg) = udp_rx.try_iter().next() { }
        while let Some(event) = view.window.poll_event() {
            match event {
                Event::Closed => view.window.close(),
                _ => ()
            }
        }
        view.render();
    }
    Ok(())
}