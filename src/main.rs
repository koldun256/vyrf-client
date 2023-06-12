use std::env;
use sfml::window::Event;
use udpclient::{ServerMsg, ClientMsg};
use view::View;

mod udpclient;
mod view;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (udp_tx, udp_rx) = udpclient::connect(&args[1]);

    let mut val = 0;
    let resources = View::load_resources();
    let mut view = View::init(&resources);

    while view.window.is_open() {
        while let Some(msg) = udp_rx.try_iter().next() {
            match msg {
                ServerMsg::Val(new_val) => val = new_val
            }
        }
        while let Some(event) = view.window.poll_event() {
            match event {
                Event::Closed => view.window.close(),
                Event::MouseButtonPressed { button: _, x: _, y: _ } => {
                    udp_tx.send(ClientMsg::Increment);
                },
                _ => ()
            }
        }
        view.render(&val);
    }
    Ok(())
}