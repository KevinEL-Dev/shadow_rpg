use std::io;
#[derive(Debug)]
enum GameErrors {
    Io(io::Error),
}
impl From<io::Error> for GameErrors {
    fn from(e: io::Error) -> Self {
        GameErrors::Io(e)
    }
}
enum States {
    Alive,
    Dead,
}
struct Player {
    name: String,
    current_state: States,
    health: i32,
    attack: i32,
}
struct Enemy {
    name: String,
    current_state: States,
    health: i32,
    attack: i32,
}
impl Player {
    fn take_damage(&mut self, damage_number: i32) {
        self.health -= damage_number;
        if self.health <= 0 {
            self.current_state = States::Dead;
        }
    }
    fn init_player(name: String) -> Self {
        Self {
            name,
            health: 10,
            attack: 5,
            current_state: States::Alive,
        }
    }
}
impl Enemy {
    fn take_damage(&mut self, damage_number: i32) {
        self.health -= damage_number;
        if self.health <= 0 {
            self.current_state = States::Dead;
        }
    }
}
fn main() {
    let test_user = Player {
        name: String::from("kevin"),
        current_state: States::Alive,
        health: 10,
        attack: 5,
    };
    /*println!(
        "player name: {}, health: {}, attack: {}",
        test_user.name, test_user.health, test_user.attack
    );*/
    // lets start implementing the combat system
    let player_name = get_player_name();
    let new_player = match player_name {
        Ok(name) => Player::init_player(name),
        Err(error) => panic!("failed to get player name {error:?}"),
    };
    println!("player: {} has been succesfully created!", new_player.name);
    println!(
        "Your stats:\nHealth: {},Attack: {}",
        new_player.health, new_player.attack
    );
}
fn get_player_name() -> Result<String, GameErrors> {
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name)?;
    Ok(player_name.trim().to_string())
}
