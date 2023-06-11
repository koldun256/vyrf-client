use std::net::UdpSocket;
use std::io::stdin;
use sfml::window::{Event, Style};
use sfml::graphics::{Text, Font, RenderWindow, RenderTarget, Color};

fn main() -> std::io::Result<()> {
    let mut port = String::new();
    stdin().read_line(&mut port)?;
    port.pop();
    let socket = UdpSocket::bind(format!("127.0.0.1:{port}"))?;
    let server = "127.0.0.1:8080";
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
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::MouseButtonPressed { button: _, x: _, y: _ } => {
                    println!("pressed!");
                    socket.send_to(&[1], server).expect("cannot send msg");
                    let mut answer = [0; 4];
                    socket.recv_from(&mut answer)?;
                    val = i32::from_be_bytes(answer);
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
    Ok(())
}