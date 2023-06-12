use position::Position;

use crate::udp_client::ServerMsg;
pub mod position;
trait GameObject {
    fn get_position(&self) -> Option<&Position>;
    fn get_id(&self) -> &u8;
}

struct Player {
    position: Position,
    id: u8,
    is_me: bool
}

impl GameObject for Player {
    fn get_id(&self) -> &u8 { &self.id }
    fn get_position(&self) -> Option<&Position> { Some(&self.position) }
}

#[repr(u8)]
enum GameObjectKind {
    Player(Player) = 1
}

pub struct Game {
    game_objects: Vec<GameObjectKind>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            game_objects: Vec::new()
        }
    }
    pub fn process_msg(&mut self, msg: ServerMsg) {
        match msg {
            ServerMsg::AddObject { id, kind } => {
                if kind == 1 {
                    self.game_objects.push(GameObjectKind::Player(Player {
                        id,
                        position: (0, 0).into(),
                        is_me: false
                    }));
                    println!("adding player with id {}", id);
                } else {
                    println!("unknown object kind");
                }
            },
            ServerMsg::BindPlayer { id } => {
                for game_object in &mut self.game_objects {
                    if let GameObjectKind::Player(player) = game_object {
                        if player.id == id {
                            player.is_me = true;
                            println!("i am player with id {}", player.id);
                        }
                    }
                }
            }
        }
    }
}