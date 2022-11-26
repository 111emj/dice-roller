use regex::{
    Regex,
    Match,
};
use std::collections::LinkedList;

fn main() {
    let args:Vec<String> = std::env::args().collect();

    for arg in &args[1..]{
        println!("{}: {}", arg, reduce_expression(arg));
    }
}

fn _roll_die(sides: u16) -> u16{
    random_number::random!(1..sides)
}

fn reduce_expression(expression: &String) -> String{
    let mut exp = expression.clone();

    exp.replace_regex(
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
    
    exp
}

trait ReplaceRegex{
    fn replace_regex<F>(&mut self, regex: &str, mutation: F)
        where F: Fn(&str) -> String;
}
impl ReplaceRegex for String{
    fn replace_regex<F>(&mut self,regex: &str,mutation: F)
    where F: Fn(&str) -> String{

        let mut overwrite = self.clone();

        let mut matches: LinkedList<Match> = Regex::new(regex)
            .expect("Invalid Regex")
            .find_iter(&self)
            .collect();

        while matches.len()!=0{
            let find = matches.pop_back().unwrap();

            let mutation = mutation(find.as_str());

            overwrite.replace_range(
                find.start()..find.end(),
                mutation.as_str());
        }

        *self = overwrite;
    }
}
