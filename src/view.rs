use sfml::SfBox;

use sfml::window::Style;
use sfml::graphics::{Text, Font, RenderWindow, RenderTarget, Color};

pub struct View<'a> {
    pub window: RenderWindow,
    counter: Text<'a>,
}

pub struct Resources {
    counter_font: SfBox<Font>
}

impl View<'_> {
    pub fn load_resources() -> Resources {
        Resources { 
            counter_font: Font::from_file("src/resources/Lato-Regular.ttf")
                                .expect("failed to read font") 
        }
    }
    pub fn init<'a>(resources: &'a Resources) -> View<'a> {
        View {
            window: RenderWindow::new((800, 600), "SFML window", Style::CLOSE, &Default::default()),
            counter: Text::new("asdf", &resources.counter_font, 16)
        }
    }

    pub fn render(&mut self, val: &i32) {
        self.window.set_active(true);
        self.window.clear(Color::rgb(50, 200, 50));
    
        self.counter.set_string(&val.to_string());
        self.window.draw(&self.counter);
    
        self.window.display();
    }
}