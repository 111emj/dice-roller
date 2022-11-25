fn main() {
    // let args:Vec<String> = std::env::args().collect();
}

fn roll_die(sides: u8) -> u8{
    random_number::random!(1..sides)
}
