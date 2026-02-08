use rand::Rng;
use std::io;
use std::num::ParseIntError;
#[derive(Debug)]
enum GameErrors {
    Io(io::Error),
    Parse(ParseIntError),
    InvalidWeapon,
    InvalidAction,
}
impl From<io::Error> for GameErrors {
    fn from(e: io::Error) -> Self {
        GameErrors::Io(e)
    }
}
impl From<ParseIntError> for GameErrors {
    fn from(e: ParseIntError) -> Self {
        GameErrors::Parse(e)
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
    D20,
}
enum MainActions {
    Attack,
}
enum ArmorType {
    NoArmor(i32),
    LightArmor(i32),
    MediumArmor(i32),
    HeavyArmor(i32),
}
#[derive(PartialEq)]
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
    armor: ArmorType,
}
struct Enemy {
    name: String,
    current_state: States,
    health: i32,
    attack: i32,
    weapon: Weapons,
    armor: ArmorType,
}
struct Weapons {
    weapon_type: WeaponTypes,
    die: Dies,
}
impl Weapons {
    fn get_die(&self) -> &Dies {
        &self.die
    }
    fn init_weapon(weapon_type: WeaponTypes, die: Dies) -> Self {
        Self { weapon_type, die }
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
            armor: ArmorType::NoArmor(10),
            current_state: States::Alive,
            weapon: Weapons {
                weapon_type: WeaponTypes::Club,
                die: Dies::D4,
            },
        }
    }
    fn change_weapon(&mut self, new_weapon: Weapons) {
        // this transfers ownership i believe
        self.weapon = new_weapon;
    }
    fn attack(&self, enemy: &mut Enemy) {
        //if roll > enemy ac attack hits, if not it miss and no damage
        let attack_roll = roll_die(&Dies::D20);

        if attack_roll >= enemy.get_armor_class() {
            println!(
                "{} attack hit! you rolled a {} which is greater than enemy's ac of {}",
                self.name,
                attack_roll,
                enemy.get_armor_class()
            );
            let damage_roll = roll_die(self.weapon.get_die());
            // have a separte function that will take the Die enum and pass that as a argument
            enemy.take_damage(damage_roll);
        } else {
            println!(
                "oh no {} missed. you rolled a {} which is less than {}",
                self.name,
                attack_roll,
                enemy.get_armor_class()
            );
        }
    }
    fn do_action(&mut self, action: MainActions, enemy: &mut Enemy) {
        match action {
            MainActions::Attack => self.attack(enemy),
        }
    }
    fn get_health(&self) -> i32 {
        self.health
    }
    fn get_armor_class(&self) -> i32 {
        match self.armor {
            ArmorType::NoArmor(armor_base) => armor_base,
            _ => 10,
        }
    }
    fn get_current_state(&self) -> States {
        match self.current_state {
            States::Alive => States::Alive,
            States::Dead => States::Dead,
        }
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
    fn attack(&self, player: &mut Player) {
        //if roll > enemy ac attack hits, if not it miss and no damage
        let attack_roll = roll_die(&Dies::D20);

        if attack_roll >= player.get_armor_class() {
            println!(
                "{} attack hit! you rolled a {} which is greater than enemy's ac of {}",
                self.name,
                attack_roll,
                player.get_armor_class()
            );
            let damage_roll = roll_die(self.weapon.get_die());
            player.take_damage(damage_roll);
        } else {
            println!(
                "oh no {} missed. you rolled a {} which is less than {}",
                self.name,
                attack_roll,
                player.get_armor_class()
            );
        }
    }
    fn _do_action(&mut self, action: MainActions, player: &mut Player) {
        match action {
            MainActions::Attack => self.attack(player),
        }
    }
    fn get_health(&self) -> i32 {
        self.health
    }
    fn get_armor_class(&self) -> i32 {
        match self.armor {
            ArmorType::NoArmor(armor_base) => armor_base,
            _ => 10,
        }
    }
    fn get_current_state(&self) -> States {
        match self.current_state {
            States::Alive => States::Alive,
            States::Dead => States::Dead,
        }
    }
}
fn main() {
    let mut test_enemy = Enemy {
        name: String::from("goblin"),
        current_state: States::Alive,
        health: 8,
        attack: 5,
        weapon: Weapons {
            weapon_type: WeaponTypes::Club,
            die: Dies::D4,
        },
        armor: ArmorType::NoArmor(10),
    };
    // player selects name
    let player_name = get_player_name();
    let mut new_player = match player_name {
        Ok(name) => Player::init_player(name),
        Err(error) => panic!("failed to get player name {error:?}"),
    };
    println!("player: {} has been succesfully created!", new_player.name);

    // player selects weapon
    let player_weapon = get_player_starter_weapon();
    match player_weapon {
        Ok(weapon) => new_player.change_weapon(weapon),
        Err(error) => panic!("please enter a valid starter weapon. club,mace,greatclub {error:?}"),
    };

    // testing if weapon is selected
    println!(
        "{} has the weapon {}",
        new_player.name,
        print_weapon_type(&new_player.weapon.weapon_type)
    );

    while new_player.get_current_state() == States::Alive
        && test_enemy.get_current_state() == States::Alive
    {
        let player_action = get_action();
        match player_action {
            Ok(action) => new_player.do_action(action, &mut test_enemy),
            Err(error) => panic!("something went wrong {error:?}"),
        }
        test_enemy.attack(&mut new_player);
    }
    println!("{} has {} health", new_player.name, new_player.get_health());
    println!("{} has {} health", test_enemy.name, test_enemy.get_health());
}
fn get_player_name() -> Result<String, GameErrors> {
    println!("Please enter the name of your player");
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name)?;
    Ok(player_name.trim().to_string())
}
fn get_player_starter_weapon() -> Result<Weapons, GameErrors> {
    println!("Oh what weapon shall you choose?");
    println!("Please enter one of the following\nclub\nmace\ngreatclub");
    let mut player_weapon = String::new();
    io::stdin().read_line(&mut player_weapon)?;
    match player_weapon.trim() {
        "club" => Ok(Weapons::init_weapon(WeaponTypes::Club, Dies::D4)),
        "greatclub" => Ok(Weapons::init_weapon(WeaponTypes::Greatclub, Dies::D8)),
        "mace" => Ok(Weapons::init_weapon(WeaponTypes::Mace, Dies::D6)),
        _ => Err(GameErrors::InvalidWeapon),
    }
}
fn get_action() -> Result<MainActions, GameErrors> {
    println!("\n[ ACTION ]");
    println!("1) Attack");
    println!(">");
    let mut player_action = String::new();
    io::stdin().read_line(&mut player_action)?;
    let player_action = player_action.trim();
    let player_action_number = player_action.parse()?;
    match player_action_number {
        1 => Ok(MainActions::Attack),
        _ => Err(GameErrors::InvalidAction),
    }
}
fn roll_die(die: &Dies) -> i32 {
    match die {
        Dies::D4 => rand::thread_rng().gen_range(1..=4),
        Dies::D6 => rand::thread_rng().gen_range(1..=6),
        Dies::D8 => rand::thread_rng().gen_range(1..=8),
        Dies::D20 => rand::thread_rng().gen_range(1..=20),
    }
}
fn print_weapon_type(weapon_type: &WeaponTypes) -> String {
    match weapon_type {
        WeaponTypes::Club => String::from("Club"),
        WeaponTypes::Mace => String::from("Mace"),
        WeaponTypes::Greatclub => String::from("GreatClub"),
    }
}
