use sfml::SfBox;

use sfml::window::Style;
use sfml::graphics::{Text, Font, RenderWindow, RenderTarget, Color};

use crate::game::Game;

pub struct View {
    pub window: RenderWindow,
}

pub struct Resources { font: SfBox<Font> }

impl View {
    pub fn load_resources() -> Resources {
        Resources { 
            font: Font::from_file("src/resources/Lato-Regular.ttf")
                                .expect("failed to read font") 
        }
    }
    pub fn init(resources: &Resources) -> View {
        View {
            window: RenderWindow::new((800, 600), "SFML window", Style::CLOSE, &Default::default()),
        }
    }

    pub fn render(&mut self, game: &Game) {
        self.window.set_active(true);
        self.window.clear(Color::BLACK);
        for obj in &game.game_objects {
            match obj {
                crate::game::GameObjectKind::Player(player) => self.window.draw(&player.drawable)
            }
        }
        self.window.display();
    }
}