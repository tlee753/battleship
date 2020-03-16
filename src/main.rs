use rand::Rng;

fn main() {
    println!("Let the games begin...");

    let mut field = [[0; 10]; 10];

    let x = rand::thread_rng().gen_range(0, 10);
    let y = rand::thread_rng().gen_range(0, 10);
    let dir = rand::thread_rng().gen_range(0, 4);

    println!();
    println!("x is {}", x + 1);
    println!("y is {}", y + 1);
    println!("dir is {}", dir);

    match dir {
        0 | 1 | 2 | 3 => {
            if x < 5 {
                for i in 0..5 {
                    field[y][x + i] = 1;
                }
            } else {
                field[y][x] = 1;
            }
        }
        _ => {
            println!("Direction not allowed");
        }
    }

    println!();
    for i in field.iter() {
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
    println!();

    println!("I sunk your battleship!");
}

// fn placeBoat(size-of-boat, boat-id)
