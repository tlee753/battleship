use rand::Rng;
use termion::color;
use termion::style;

fn main() {
    println!("\nLet the games begin...");

    let mut game: Game = Game { field_1: [[0; 10]; 10], field_2: [[0; 10]; 10] };
    game.init_game();

    println!("I sunk your battleship!\n");
}

struct Game {
    field_1: [[u32; 10]; 10],
    field_2: [[u32; 10]; 10],
}

impl Game {
    fn init_game(&mut self) {

        for i in 1..2 {
            self.place_boat(i);
        }
        
        // self.place_boat(1);

        self.print_field();
    }

    fn place_boat(&mut self, ship_id: u32) {

        let ships: [u32; 5] = [5, 4, 3, 3, 2];
        let ship_length: u32 = ships[(ship_id - 1) as usize];

        let mut placed = false;
        let mut x: u32;
        let mut y: u32;
        let mut dir: bool;

        while !placed {

            x = rand::thread_rng().gen_range(0, 10);
            y = rand::thread_rng().gen_range(0, 10);
            dir = rand::random();

            println!("x is {}", x);
            println!("y is {}", y);
            println!("dir is {}", dir);
            println!("ship length is {}", ship_length);
            println!("");

            if dir {
                if (x <= 10 - ship_length) && (self.field_1[y as usize][x as usize..x as usize + ship_length as usize].iter().sum::<u32>() == 0) {
                    for i in 0..ship_length {
                        self.field_1[y as usize][(x+i) as usize] = ship_id;
                    }
                    placed = true;
                }
            } else {
                if (y <= 10 - ship_length) && (self.field_1[y as usize..y as usize + ship_length as usize][x as usize].iter().sum::<u32>() == 0) {
                    for i in 0..ship_length {
                        self.field_1[(y+i) as usize][x as usize] = ship_id;
                    }
                    placed = true;
                }
            }

        }
    }

    fn print_field(&self) {
        println!();

        for i in self.field_1.iter() {
            for j in i {
                if j > &0 {
                    print!("{}{}{} ", color::Fg(color::Green), j, style::Reset);
                } else {
                    print!("{} ", j);
                }
            }
            println!();
        }

        println!();
    }
}
