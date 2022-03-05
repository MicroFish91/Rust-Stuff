#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

#[derive(Debug)]
struct Grapes {
    number_left: i32,
}

trait Bite {
    fn bite(self: &mut Self);
}

// trait Bite {
//     fn bite(self: &mut Self) {
//         println!("Default bite behavior.");
//     }
// }

// impl Bite for Carrot {}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left *= 0.8;
    }
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.number_left -= 1;
    }
}

fn main() {
    let mut carrot = Carrot { percent_left: 100.0 };
    let mut grapes = Grapes { number_left: 100 };

    carrot.bite();
    println!("I take a bite: {:?}", carrot);
    println!("{}% of carrot left.", carrot.percent_left);

    grapes.bite();
    println!("Eat a grape: {:?}", grapes);
    println!("{} grapes left.", grapes.number_left);

    bunny_nibbles(&mut carrot);
    bunny_nibbles(&mut grapes);

    println!("{}% of carrot left.", carrot.percent_left);
    println!("{} grapes left.", grapes.number_left);
}

fn bunny_nibbles<T: Bite>(food: &mut T) {
    food.bite();
    food.bite();
    food.bite();
    food.bite();
    println!("Bunny nibbles...");
}
