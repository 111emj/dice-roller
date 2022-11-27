use regex::{
    Regex,
    Match,
};
use std::collections::LinkedList;
use random_number::random;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

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
                let y = x
                    .strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .to_string();
                reduce_expression(&y)
        }
    );

    regex_replace( // expand dice roll expressions to their results
        &mut exp,
        r"\d*d\d*",
        |x: &str|{
            let (qt,ql) = x.split_once("d").unwrap();
            format!("{:?}s",
                roll_dice(
                    qt.parse::<u16>()
                        .unwrap_or(1),
                    ql.parse::<u16>()
                        .expect("unknown dice quality")
                )
            )
        }
    );
    
    regex_replace( // reduce generic dice roll results to their sum
        &mut exp,
        r"\[.*\]s",
        |x: &str|{
            x
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .split(",")
                .map(|string|
                     string.trim()
                     .parse::<u16>()
                     .expect("list of rolls contained non-integer(u16) element(s)")
                 )
                .collect::<Vec<u16>>()
                .iter()
                .sum::<u16>()
                .to_string()
        }
    );

    regex_replace( // reduce multiplications
        &mut exp,
        r"(\d{1,}\*){1,}\d{1,}",
        |x: &str|{
            x
                .split("*")
                .map(|a|
                    a
                    .parse::<u16>()
                    .unwrap()
                )
                .collect::<Vec<u16>>()
                .iter()
                .product::<u16>()
                .to_string()

        }
    );

    regex_replace( // reduce division
        &mut exp,
        r"(\d{1,}/){1,}\d{1,}",
        |x: &str|{
            let mut list = x
                .split("/")
                .map(|a|
                    a
                    .parse::<u16>()
                    .unwrap()
                )
                .collect::<LinkedList<u16>>();
            let dividend = list
                .pop_front()
                .unwrap();
            (dividend/list
                .iter()
                .product::<u16>()
                )
                .to_string()
        }
    );

    regex_replace( // reduce addition
        &mut exp,
        r"(\d{1,}\+){1,}\d{1,}",
        |x: &str|{
            x
                .split("+")
                .map(|a|
                    a
                    .parse::<u16>()
                    .unwrap()
                )
                .collect::<Vec<u16>>()
                .iter()
                .sum::<u16>()
                .to_string()

        }
    );
    
    exp
}

fn regex_replace<F>(source: &mut String,regex: &str,mutation: F)// replaces all portions of text in source that match regex with output of mutation
where F: Fn(&str) -> String{

    let mut overwrite = source.clone();

    let mut matches: LinkedList<Match> = Regex::new(regex)
        .expect("Invalid Regex")
        .find_iter(&source)
        .collect();
    // println!("{:?}",matches);

    while matches.len()!=0{
        let find = matches.pop_back().unwrap();

        let mutation = mutation(find.as_str());

        overwrite.replace_range(
            find.start()..find.end(),
            mutation.as_str());
    }

    *source = overwrite;
}
