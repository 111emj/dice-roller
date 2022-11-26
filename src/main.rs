// use regex::Regex;

fn main() {
    let args:Vec<String> = std::env::args().collect();

    for arg in &args[1..]{
        println!("{}",arg);
    }
}

fn _roll_die(sides: u16) -> u16{
    random_number::random!(1..sides)
}

fn _reduce_expression(expression: String) -> u16{
    let exp = expression.clone();

    // let parentheses_regex = Regex::new("(.*)").unwrap();
    
    exp.parse::<u16>().unwrap()
}
