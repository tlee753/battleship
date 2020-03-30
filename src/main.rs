use rand::Rng;

fn main() {
    println!("\nLet the games begin...\n");

    let mut game: Game = Game { field_1: [[0; 10]; 10], field_2: [[0; 10]; 10] };
    game.init_game();

    println!("\nI sunk your battleship!\n");
}

struct Game {
    field_1: [[i32; 10]; 10],
    field_2: [[i32; 10]; 10],
}

impl Game {
    fn init_game(&mut self) {

        // for i in 1..6 {
        //     self.place_boat(i);
        // }
        
        self.place_boat(1);

        self.print_field();
    }

    fn place_boat(&mut self, ship_id: i32) {

        let ships: [i32; 5] = [5, 4, 3, 3, 2];
        let ship_length: i32 = ships[(ship_id - 1) as usize];

        let mut placed = false;
        let mut x: i32;
        let mut y: i32;
        let mut dir: i32;

        while !placed {

            x = rand::thread_rng().gen_range(0, 10);
            y = rand::thread_rng().gen_range(0, 10);
            dir = rand::thread_rng().gen_range(0, 4);

            println!("x is {}", x);
            println!("y is {}", y);
            println!("dir is {}", dir);

            match dir {
                0 | 1 | 2 | 3 => {
                    if (x <= 10 - ship_length) && (self.field_1[y as usize][x as usize..x as usize + ship_length as usize].iter().sum::<i32>() == 0) {
                        for i in 0..ship_length {
                            self.field_1[y as usize][(x+i) as usize] = ship_id;
                        }
                        placed = true;
                    }
                }
                _ => {
                    println!("Direction not found!");
                }
            }
        }
    }

    fn print_field(&self) {
        println!();

        for i in self.field_1.iter() {
            for j in i {
                print!("{} ", j);
            }
            println!();
        }

        println!();
    }
}
