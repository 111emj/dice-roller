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
    if args.len()>1{
        for arg in &args[1..]{
            let mut arg = arg.clone();
            arg.retain(|c|!c.is_whitespace());
            match arg
                    .split_once("x"){
                Some(repetitions_expression) =>{
                    match repetitions_expression
                            .0
                            .trim()
                            .parse::<u16>(){
                        Ok(val) =>{
                            for _ in 0..val{
                                println!("{}: {}", 
                                     repetitions_expression.1,
                                     reduce_expression(&repetitions_expression
                                       .1
                                       .to_string()));
                            }
                        },
                        Err(_) =>{
                            println!("{}: {}",arg, reduce_expression(&arg));
                        }
                    }
                },
                None =>{
                    println!("{}: {}",arg, reduce_expression(&arg));
                }
            }
        }
    }
    else {
        loop{
            // read input into a string and remove whitespace
            let mut line = String::new();
            stdin()
                .read_line(&mut line)
                .expect("Failed to Access Input Stream");
            line.retain(|c|!c.is_whitespace());

            // check for specifictaion of expression repetition and execute it if required
            match line
                    .split_once("x"){
                Some(repetitions_expression) =>{
                    match repetitions_expression
                            .0
                            .trim()
                            .parse::<u16>(){
                        Ok(val) =>{
                            for _ in 0..val{
                                println!(": {}", 
                                     reduce_expression(&repetitions_expression
                                       .1
                                       .to_string()));
                            }
                        },
                        Err(_) =>{
                            println!(": {}", reduce_expression(&line));
                        }
                    }
                },
                None =>{
                    if line
                            .to_lowercase()
                            .starts_with("q"){
                        break;
                    }

                    println!(": {}", reduce_expression(&line));
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
            format!("{:?}",
                roll_dice(
                    qt.parse::<u16>()
                        .unwrap_or(1),
                    ql.parse::<u16>()
                        .expect("unknown dice quality")
                )
            )
        }
    );

    while 0 != Regex::new(r"\[.*][_\^\d]")
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
                    .map(|a|{
                        a
                            .trim()
                            .parse::<u16>()
                            .unwrap()
                    })
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
                    .map(|a|{
                        a
                            .trim()
                            .parse::<u16>()
                            .unwrap()
                    })
                    .collect::<Vec<u16>>();
                list.sort_unstable_by(|a,b|a.cmp(b).reverse());
                format!("{:?}",list)
            }
        );

        regex_replace( // truncate dice roll list to this specified size
            &mut exp,
            r"\[[^]]*]\d{1,}",
            |x: &str|{
                let len = x.split_once("]").unwrap().1.parse::<usize>().unwrap();
                let mut list = x
                    .strip_prefix("[")
                    .unwrap()
                    .strip_suffix(format!("]{}",len).as_str())
                    .unwrap()
                    .split(",")
                    .map(|a|a
                         .trim()
                         .parse::<u16>()
                         .unwrap()
                    )
                    .collect::<LinkedList<u16>>();
                let mut out: LinkedList<u16> = LinkedList::new();
                
                for _ in 0..len{
                    match list.pop_front(){
                        Some(val) => {
                            out.push_back(val)
                        }
                        None => {break;}
                    }
                }
               
                format!("{:?}",out)
            }
        );    
    }
    

    regex_replace( // reduce generic dice roll results to their sum
        &mut exp,
        r"\[[^]]*]",
        |x: &str|
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

    // println!("{}",overwrite);
    *source = overwrite;
}
