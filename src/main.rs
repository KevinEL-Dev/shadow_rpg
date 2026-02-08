use rand::Rng;
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
enum WeaponTypes {
    Club,
    Greatclub,
    Mace,
}
enum Dies {
    D4,
    D6,
    D8,
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
    weapon: Weapons,
}
struct Enemy {
    name: String,
    current_state: States,
    health: i32,
    attack: i32,
    weapon: Weapons,
}
struct Weapons {
    weapon_type: WeaponTypes,
    die: Dies,
}
impl Weapons {
    fn get_die(&self) -> &Dies {
        &self.die
    }
}
impl Player {
    fn take_damage(&mut self, damage_number: i32) {
        self.health -= damage_number;
        println!("{} has took {} damage!", self.name, damage_number);
        if self.health <= 0 {
            self.current_state = States::Dead;
            println!("{} has died", self.name);
        }
    }
    fn init_player(name: String) -> Self {
        Self {
            name,
            health: 10,
            attack: 5,
            current_state: States::Alive,
            weapon: Weapons {
                weapon_type: WeaponTypes::Club,
                die: Dies::D4,
            },
        }
    }
    fn attack(&self, enemy: &mut Enemy) {
        // get the weapon from the struct.

        let attack_roll = roll_die(self.weapon.get_die());
        // have a separte function that will take the Die enum and pass that as a argument
        enemy.take_damage(attack_roll);
        // that function will then use the range from 1-N (DN). probably use a match in this case
        // what should we return from that function? a int between 1-N.
    }
}
impl Enemy {
    fn take_damage(&mut self, damage_number: i32) {
        self.health -= damage_number;
        println!("{} has took {} damage!", self.name, damage_number);
        if self.health <= 0 {
            self.current_state = States::Dead;
            println!("{} has died", self.name);
        }
    }
    fn attack(&self, enemy: &mut Player) {
        // get the weapon from the struct.

        let attack_roll = roll_die(self.weapon.get_die());
        // have a separte function that will take the Die enum and pass that as a argument
        enemy.take_damage(attack_roll);
        // that function will then use the range from 1-N (DN). probably use a match in this case
        // what should we return from that function? a int between 1-N.
    }
}
fn main() {
    let test_user = Player {
        name: String::from("kevin"),
        current_state: States::Alive,
        health: 10,
        attack: 5,
        weapon: Weapons {
            weapon_type: WeaponTypes::Club,
            die: Dies::D4,
        },
    };
    let mut test_enemy = Enemy {
        name: String::from("goblin"),
        current_state: States::Alive,
        health: 8,
        attack: 5,
        weapon: Weapons {
            weapon_type: WeaponTypes::Club,
            die: Dies::D4,
        },
    };
    /*println!(
        "player name: {}, health: {}, attack: {}",
        test_user.name, test_user.health, test_user.attack
    );*/
    // lets start implementing the combat system
    let player_name = get_player_name();
    let mut new_player = match player_name {
        Ok(name) => Player::init_player(name),
        Err(error) => panic!("failed to get player name {error:?}"),
    };
    println!("player: {} has been succesfully created!", new_player.name);
    println!(
        "Your stats:\nHealth: {},Attack: {}",
        new_player.health, new_player.attack
    );
    new_player.attack(&mut test_enemy);
    test_enemy.attack(&mut new_player);
}
fn get_player_name() -> Result<String, GameErrors> {
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name)?;
    Ok(player_name.trim().to_string())
}
fn roll_die(die: &Dies) -> i32 {
    match die {
        Dies::D4 => rand::thread_rng().gen_range(1..=4),
        Dies::D6 => rand::thread_rng().gen_range(1..=6),
        Dies::D8 => rand::thread_rng().gen_range(1..=8),
    }
}
