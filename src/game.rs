use position::Position;
use sfml::{graphics::{RectangleShape, Transformable, Shape, Color}, system::Vector2};

use crate::udp_client::ServerMsg;
pub mod position;
trait GameObject {
    fn get_position(&self) -> Option<&Position>;
    fn get_id(&self) -> &u8;
}

#[derive(Debug)]
pub struct Player<'a> {
    position: Position,
    id: u8,
    is_me: bool,
    pub drawable: RectangleShape<'a>
}

impl GameObject for Player<'_> {
    fn get_id(&self) -> &u8 { &self.id }
    fn get_position(&self) -> Option<&Position> { Some(&self.position) }
}

pub enum GameObjectKind<'a> {
    Player(Player<'a>)
}

pub struct Game<'a> {
    pub game_objects: Vec<GameObjectKind<'a>>,
}

impl Game<'_> {
    pub fn new() -> Self {
        Game {
            game_objects: Vec::new()
        }
    }
    pub fn process_msg(&mut self, msg: ServerMsg) {
        match msg {
            ServerMsg::AddObject { id, kind, x, y} => {
                if kind == 1 {
                    let mut drawable = RectangleShape::new();
                    drawable.set_size(Vector2::new(50f32, 50f32));
                    drawable.set_position(Vector2::new(x as f32, y as f32));
                    drawable.set_fill_color(Color::WHITE);
                    let obj = Player {
                        id,
                        position: (x, y).into(),
                        is_me: false,
                        drawable
                    };
                    println!("adding player {:?}", obj);
                    self.game_objects.push(GameObjectKind::Player(obj));
                } else {
                    println!("unknown object kind");
                }
            },
            ServerMsg::BindPlayer { id } => {
                for game_object in &mut self.game_objects {
                    if let GameObjectKind::Player(player) = game_object {
                        if player.id == id {
                            player.is_me = true;
                            player.drawable.set_fill_color(Color::GREEN);
                            println!("i am player with id {}", player.id);
                        }
                    }
                }
            }
        }
    }
}