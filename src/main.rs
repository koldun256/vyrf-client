use std::env;
use std::sync::mpsc::{Receiver, Sender};
use sfml::window::{Event, Style};
use sfml::graphics::{Text, Font, RenderWindow, RenderTarget, Color};
use udpclient::ServerMsg;

pub mod udpclient;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (udp_tx, udp_rx) = udpclient::connect(&args[1]);

    let mut val = 0;

    let mut window = RenderWindow::new((800, 600),
                             "SFML window",
                             Style::CLOSE,
                             &Default::default());
    let font = Font::from_file("src/resources/Lato-Regular.ttf").expect("failed to read font");
    let mut text = Text::new(&val.to_string(), &font, 16);

    while window.is_open() {
        while let Some(msg) = udp_rx.try_iter().next() {
            match msg {
                ServerMsg::Val(new_val) => val = new_val
            }
        }
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::MouseButtonPressed { button: _, x: _, y: _ } => {
                    udp_tx.send(udpclient::ClientMsg::Increment);
                },
                _ => ()
            }
        }
        window.set_active(true);
        window.clear(Color::rgb(50, 200, 50));
    
        text.set_string(&val.to_string());
        window.draw(&text);
    
        window.display();
    }
    Ok(())
}