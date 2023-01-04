use regex::{
    Regex,
    Match,
};
use std::{
    collections::LinkedList,
    io::stdin,
};
use random_number::random;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len()>1{ // run once for each argument
        for arg in args[1..].iter(){
            match arg.chars()
                    .filter(|x| !x.is_whitespace())
                    .collect::<String>()
                    .split_once("x"){
                None => println!("{}: {}",arg, reduce_expression(&arg)),

                Some(expression) =>{
                    println!("{}",arg);
                    for _ in 0..
                            expression.0
                            .parse::<u16>()
                            .unwrap_or(1){
                        println!(": {}", reduce_expression(&expression.1.to_string()))
                    }
                }
            }
        }
    }
    else { // if no arguments provided run in interactive mode
        loop{
            // read input into a string and remove whitespace
            let mut line = String::new();
            stdin()
                .read_line(&mut line)
                .expect("Failed to Access Input Stream");
            let line = line.chars()
                .filter(|x| !x.is_whitespace())
                .collect::<String>();

            if line.starts_with("q") {
                break;
            }

            // check for specifictaion of expression repetition and execute it if required
            match line.split_once("x"){
                None => println!(": {}", reduce_expression(&line)),
                Some(expression) =>{
                    for _ in 0..
                            expression.0
                            .parse::<u16>()
                            .unwrap_or(1){
                        println!(": {}", reduce_expression(&expression.1.to_string()))
                    }
                }
            }
        }
    }
}

fn roll_dice(quantity: u16, quality: u16) -> LinkedList<u16>{
    (0..quantity).map(|_|random!(1..quality+1)).collect()
}

fn reduce_expression(expression: &String) -> String{
    let mut exp = expression.clone();

    regex_replace( // reduce parentheses and recur inside of each set of them
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
        r"\d*d\d{1,}",
        |x: &str|{
            let (qt,ql) = x.split_once("d").unwrap();
            let qt = qt
                .parse::<u16>()
                .unwrap_or(1);
            let ql = ql
                .parse::<u16>()
                .unwrap();

            format!("{}:{:?}",ql,roll_dice(qt,ql))
        }
    );

    while 0 != Regex::new(r"\[.*][_\^\d(rr)]") // loop sorting and indexing expressions
            .unwrap()
            .find_iter(&exp)
            .collect::<Vec<Match>>()
            .len(){
        regex_replace( // sort dice roll results by increasing size (if necessary)
            &mut exp,
            r"\[[^]]*]_",
            |x: &str|{
                let mut list = x
                    .strip_prefix("[")
                    .unwrap()
                    .strip_suffix("]_")
                    .unwrap()
                    .split(",")
                    .map(|a|
                        a
                            .trim()
                            .parse::<u16>()
                            .unwrap()
                        )
                    .collect::<Vec<u16>>();
                list.sort_unstable();
                format!("{:?}",list)
            }
        );
        
        regex_replace( // sort dice roll results by decreasing size (if necessary)
            &mut exp,
            r"\[[^]]*]\^",
            |x: &str|{
                let mut list = x
                    .strip_prefix("[")
                    .unwrap()
                    .strip_suffix("]^")
                    .unwrap()
                    .split(",")
                    .map(|a|
                        a
                            .trim()
                            .parse::<u16>()
                            .unwrap()
                        )
                    .collect::<Vec<u16>>();
                list.sort_unstable_by(|a,b|a.cmp(b).reverse());
                format!("{:?}",list)
            }
        );

        regex_replace( // truncate dice roll list to this specified size
            &mut exp,
            r"\[[^]]*]\d{1,}",
            |x: &str|{
                let (list, len) = x.split_once("]").unwrap();

                let len = len.parse::<usize>().unwrap();
                let list = list
                    .strip_prefix("[")
                    .unwrap()
                    .split(",")
                    .map(|a|a
                         .trim()
                         .parse::<u16>()
                         .unwrap()
                        )
                    .collect::<LinkedList<u16>>();

                format!("{:?}",
                        list.iter()
                            .zip(0..len)
                            .map(|x|x.0)
                            .collect::<LinkedList<&u16>>()
                        )
            }
        );    

        regex_replace( // keep only specified indeces of roll
            &mut exp,
            r"\[[^]]*]\{(\d{1,},)*\d{1,}}",
            |x|{
                let (rolls,indeces) = x.split_once("]{").unwrap();
                let rolls = rolls
                    .strip_prefix("[")
                    .unwrap()
                    .split(",")
                    .map(|roll|
                         roll
                         .trim()
                         .parse::<u16>()
                         .unwrap()
                         )
                    .collect::<Vec<u16>>();
                let indeces = indeces
                    .strip_suffix("}")
                    .unwrap()
                    .split(",")
                    .map(|index|
                         index
                         .trim()
                         .parse::<u16>()
                         .unwrap()
                         -1
                         )
                    .collect::<Vec<u16>>();
                let mut new_rolls: Vec<u16> = Vec::new();
                for (roll,index) in rolls.iter().zip(0..){
                    if indeces.contains(&index){
                        new_rolls.push(roll.clone());
                    }
                }
                format!("{:?}",new_rolls)
            }
        );
        regex_replace( // reroll specified values
            &mut exp,
            r"\d{1,}:\[[^]]*]rr\d{1,}",
            |x|{
                let (ql, leftover) = x
                    .split_once(":[")
                    .unwrap();
                let (leftover, reroll_value) = leftover
                    .split_once("]rr")
                    .unwrap();
                let ql = ql.parse::<u16>().unwrap();
                let list = leftover
                    .split(",")
                    .map(|string|{
                        let num_string = string.trim();
                        if num_string==reroll_value {
                            roll_dice(1,ql)
                                .pop_front()
                                .unwrap()
                                .to_string()
                                .parse::<u16>()
                                .unwrap()
                        }
                        else{
                            num_string
                                .to_string()
                                .parse::<u16>()
                                .unwrap()
                        }
                    })
                    .collect::<Vec<u16>>();
                 format!("{}:{:?}",ql,list)
            }
        )
    }
    

    regex_replace( // reduce generic dice roll results to their sum
        &mut exp,
        r"\d{1,}:\[[^]]*]",
        |x: &str|
            x
                .split_once(":[")
                .unwrap()
                .1
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
    );

    regex_replace( // reduce multiplications
        &mut exp,
        r"(\d{1,}\*){1,}\d{1,}",
        |x: &str|
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
        |x: &str|
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
    );

    regex_replace( // reduce subtraction
        &mut exp,
        r"(\d{1,}-){1,}\d{1,}",
        |x: &str|{
            let mut list = x
                .split("-")
                .map(|a|
                    a
                    .parse::<u16>()
                    .unwrap()
                )
                .collect::<LinkedList<u16>>();
            (list.pop_front().unwrap()-list.iter().sum::<u16>()).to_string()

        }
    );
    
    exp
}

fn regex_replace<F>(source: &mut String,regex: &str,mutation: F)
        where F: Fn(&str) -> String{
    // replaces all portions of text in source that match regex with output of mutation

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

    // println!("{}",overwrite);
    *source = overwrite;
}
