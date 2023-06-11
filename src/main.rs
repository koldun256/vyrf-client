use std::thread;
use std::sync::mpsc;
use sfml::window::{Event, Style};
use sfml::graphics::{Text, Font, RenderWindow, RenderTarget, Color};
use udpclient::ServerMsg;

pub mod udpclient;

fn main() -> std::io::Result<()> {
    let socket = udpclient::open_udp_socket();
    let (tx, rx) = mpsc::channel::<ServerMsg>();
    println!("1");
    let mut window = RenderWindow::new((800, 600),
                             "SFML window",
                             Style::CLOSE,
                             &Default::default());
    let mut val = 0;
    let font = match Font::from_file("src/resources/Lato-Regular.ttf") {
        Some(font) => font,
        None => {
            panic!("Failed to read font file!");
        }
    };
    let mut text = Text::new(&val.to_string(), &font, 16);
    thread::scope(|s| { 
        s.spawn(|| udpclient::start_udp_thread(tx, &socket)); 
        while window.is_open() {
            while let Some(msg) = rx.try_iter().next() {
                match msg {
                    ServerMsg::Val(new_val) => val = new_val
                }
            }
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed => window.close(),
                    Event::MouseButtonPressed { button: _, x: _, y: _ } => {
                        udpclient::send_udp_msg(udpclient::ClientMsg::Increment, &socket);
                    },
                    _ => ()
                }
                if event == Event::Closed { window.close(); }
            }
            window.set_active(true);
            window.clear(Color::rgb(50, 200, 50));
        
            text.set_string(&val.to_string());
            window.draw(&text);
        
            window.display();
        }
    });
    Ok(())
}