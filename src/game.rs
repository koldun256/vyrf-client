use vec2::Vec2;
use sfml::{graphics::{RectangleShape, Transformable, Shape, Color, Drawable}, system::Vector2};

use crate::udp_client::ServerMsg;
pub mod vec2;
pub struct GameObject<'a> {
    position: Vec2,
    id: u8,
    kind: GameObjectKind<'a>
}

pub enum GameObjectKind<'a> {
    Player { is_me: bool, drawable: RectangleShape<'a> }
}
impl GameObject<'_> {
    fn new_player(id: u8, pos: Vec2) -> Self {
        let mut drawable = RectangleShape::new();
        drawable.set_size(Vector2::new(50f32, 50f32));
        drawable.set_position(Vector2::new(pos.x as f32, pos.y as f32));
        drawable.set_fill_color(Color::WHITE);
        GameObject { 
            position: pos,
            id,
            kind: GameObjectKind::Player { 
                is_me: false,
                drawable
            },
        }
    }
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        match &mut self.kind {
            GameObjectKind::Player { is_me: _, ref mut drawable } => {
                drawable.set_position(Vector2::new(self.position.x as f32, self.position.y as f32));
            }
        }
    }
    pub fn drawable(&self) -> &dyn Drawable {
        match &self.kind {
            GameObjectKind::Player { is_me: _, drawable } => drawable
        }
    }
}
pub struct Game<'a> {
    pub game_objects: Vec<GameObject<'a>>,
}

impl Game<'_> {
    pub fn new() -> Self {
        Game {
            game_objects: Vec::new()
        }
    }
    pub fn process_msg(&mut self, msg: ServerMsg) {
        match msg {
            ServerMsg::AddObject { id, kind, pos} => {
                if kind == 1 {
                    self.game_objects.push(GameObject::new_player(id, pos));
                } else {
                    println!("unknown object kind");
                }
            },
            ServerMsg::BindPlayer { id } => {
                for obj in &mut self.game_objects {
                    if let GameObjectKind::Player { mut is_me, ref mut drawable } = &mut obj.kind {
                        if obj.id == id {
                            is_me = true;
                            drawable.set_fill_color(Color::GREEN);
                        }
                    }
                }
            },
            ServerMsg::SetPosition { id, pos } => {
                for obj in &mut self.game_objects {
                    if obj.id == id {
                        obj.set_position(pos)
                    }
                }
            }
        }
    }
}