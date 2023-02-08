use std::io;

enum Room {
    Hall,
    Kitchen,
    Bathroom,
    Bedroom,
}

struct Game {
    current_room: Room,
}

impl Game {
    fn new() -> Game {
        Game {
            current_room: Room::Hall,
        }
    }

    fn play(&mut self) {
        loop {
            println!("You are in the {}.", self.current_room_name());

            println!("Which direction would you like to go?");
            println!("1. North");
            println!("2. South");
            println!("3. East");
            println!("4. West");
            println!("5. Quit");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().parse::<u32>().unwrap();

            match input {
                1 => self.go_north(),
                2 => self.go_south(),
                3 => self.go_east(),
                4 => self.go_west(),
                5 => break,
                _ => println!("Invalid choice"),
            }
        }
    }

    fn current_room_name(&self) -> &str {
        match self.current_room {
            Room::Hall => "Hall",
            Room::Kitchen => "Kitchen",
            Room::Bathroom => "Bathroom",
            Room::Bedroom => "Bedroom",
        }
    }

    fn go_north(&mut self) {
        self.current_room = match self.current_room {
            Room::Hall => Room::Kitchen,
            Room::Kitchen => Room::Kitchen,
            Room::Bathroom => Room::Bathroom,
            Room::Bedroom => Room::Bedroom,
        };
    }

    fn go_south(&mut self) {
        self.current_room = match self.current_room {
            Room::Hall => Room::Hall,
            Room::Kitchen => Room::Hall,
            Room::Bathroom => Room::Bathroom,
            Room::Bedroom => Room::Bedroom,
        };
    }

    fn go_east(&mut self) {
        self.current_room = match self.current_room {
            Room::Hall => Room::Bathroom,
            Room::Kitchen => Room::Kitchen,
            Room::Bathroom => Room::Bathroom,
            Room::Bedroom => Room::Bedroom,
        };
    }

    fn go_west(&mut self) {
        self.current_room = match self.current_room {
            Room::Hall => Room::Hall,
            Room::Kitchen => Room::Kitchen,
            Room::Bathroom => Room::Hall,
            Room::Bedroom => Room::Bedroom,
        };
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
