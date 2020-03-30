use rand::Rng;
use termion::color;
use termion::style;

fn main() {
    println!("\nLet the games begin...");

    let mut game: Game = Game {
        field_1: [[0; 10]; 10],
        field_2: [[0; 10]; 10],
    };
    game.init_game();

    println!("I sunk your battleship!\n");
}

struct Game {
    field_1: [[u32; 10]; 10],
    field_2: [[u32; 10]; 10],
}

impl Game {
    fn init_game(&mut self) {
        for i in 1..6 {
            self.place_boat(i, 0);
            self.place_boat(i, 1);
        }

        self.print_fields();
    }

    fn place_boat(&mut self, ship_id: u32, field_id: u32) {
        let ships: [u32; 5] = [5, 4, 3, 3, 2];
        let ship_length: u32 = ships[(ship_id - 1) as usize];

        let mut field: [[u32; 10]; 10] = self.field_1;
        match field_id {
            0 => field = self.field_1,
            1 => field = self.field_2,
            _ => println!("No such field."),
        }

        let mut placed = false;
        let mut x: u32;
        let mut y: u32;
        let mut dir: bool;

        while !placed {
            x = rand::thread_rng().gen_range(0, 10);
            y = rand::thread_rng().gen_range(0, 10);
            dir = rand::random();

            if dir {
                // horizontal
                if (x <= 10 - ship_length)
                    && (field[y as usize][x as usize..x as usize + ship_length as usize]
                        .iter()
                        .sum::<u32>()
                        == 0)
                {
                    for i in 0..ship_length {
                        field[y as usize][(x + i) as usize] = ship_id;
                    }
                    placed = true;
                }
            } else {
                // vertical
                if y <= 10 - ship_length {
                    let mut sum: u32 = 0;
                    for i in 0..ship_length {
                        sum += field[(y + i) as usize][x as usize];
                    }
                    if sum == 0 {
                        for i in 0..ship_length {
                            field[(y + i) as usize][x as usize] = ship_id;
                        }
                        placed = true;
                    }
                }
            }
        }

        match field_id {
            0 => self.field_1 = field,
            1 => self.field_2 = field,
            _ => println!("No such field."),
        }
    }

    fn print_fields(&self) {
        println!();

        for i in 0..10 {
            for j in 0..10 {
                if self.field_1[i][j] > 0 {
                    print!(
                        "{}{}{} ",
                        color::Fg(color::Green),
                        self.field_1[i][j],
                        style::Reset
                    );
                } else {
                    print!("{} ", self.field_1[i][j]);
                }
            }
            print!("    ");

            for j in 0..10 {
                if self.field_2[i][j] > 0 {
                    print!(
                        "{}{}{} ",
                        color::Fg(color::Green),
                        self.field_2[i][j],
                        style::Reset
                    );
                } else {
                    print!("{} ", self.field_2[i][j]);
                }
            }
            println!();
        }

        println!();
    }
}
