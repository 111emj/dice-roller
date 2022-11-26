use regex::{
    Regex,
    Match,
};
use std::collections::LinkedList;
use random_number::random;

fn main() {
    let args:Vec<String> = std::env::args().collect();

    for arg in &args[1..]{
        println!("{}: {}", arg, reduce_expression(arg));
    }
}

fn roll_dice(quantity: u16, quality: u16) -> LinkedList<u16>{
    (0..quantity).map(|_|random!(1..quality)).collect()
}

fn reduce_expression(expression: &String) -> String{
    let mut exp = expression.clone();

    regex_replace( // reduce parentheses and recur inside of each set
        &mut exp,
        r"\([^\)]*[^\(]*\)",
        |x: &str|{
                let y = x.strip_prefix("(")
                    .expect("failed to mutate match")
                    .strip_suffix(")")
                    .expect("Failed to mutate match")
                    .to_string();
                reduce_expression(&y)
        }
    );

    regex_replace( // reduce simple rolls ex: 13d17
        &mut exp,
        r"\d*d\d*",
        |x: &str|{
            let (qt,ql) = x.split_once("d").unwrap();
            roll_dice(
                qt.parse::<u16>().unwrap_or(1),
                ql.parse::<u16>().expect("unknown dice quality")
            )
                .iter()
                .sum::<u16>().to_string()
        }
    );
    
    exp
}

fn regex_replace<F>(source: &mut String,regex: &str,mutation: F)
where F: Fn(&str) -> String{

    let mut overwrite = source.clone();

    let mut matches: LinkedList<Match> = Regex::new(regex)
        .expect("Invalid Regex")
        .find_iter(&source)
        .collect();

    while matches.len()!=0{
        let find = matches.pop_back().unwrap();

        let mutation = mutation(find.as_str());

        overwrite.replace_range(
            find.start()..find.end(),
            mutation.as_str());
    }

    *source = overwrite;
}
