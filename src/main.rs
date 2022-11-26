use regex::Regex;

fn main() {
    let args:Vec<String> = std::env::args().collect();

    for arg in &args[1..]{
        match reduce_expression(arg){
            Ok(result) => println!("{}: {}", arg, result),
            Err(_) => println!("Failed to parse Expression '{}'",arg),
        }
    }
}

fn _roll_die(sides: u16) -> u16{
    random_number::random!(1..sides)
}

fn reduce_expression(expression: &String) -> Result<u16,std::num::ParseIntError>{
    let mut exp = expression.clone();

    while exp.replace_regex(
            r"\([^\)]*[^\(]*\)",
            |x: &str|{
                x.strip_prefix("(")
                    .expect("failed to mutate match")
                    .strip_suffix(")")
                    .expect("Failed to mutate match")
            }
        ){}
    
    exp.parse::<u16>()
}

trait ReplaceRegex{
    fn replace_regex<F>(&mut self, regex: &str, mutation: F) -> bool
        where F: Fn(&str) -> &str;
}
impl ReplaceRegex for String{
    fn replace_regex<F>(&mut self,regex: &str,mutation: F) -> bool
    where F: Fn(&str) -> &str{

        let mut overwrite = self.clone();

        let captures = Regex::new(regex)
            .expect("Invalid Regex")
            .captures(&self);
        println!("{:?}",captures);

        if captures.is_some(){
            let captures = captures.unwrap();
            let mut counter = captures.len()-1;

            loop{
                let capture = captures.get(counter)
                    .expect(format!("failed to get capture {} of {:?}",counter,captures).as_str());

                
                let mutation = mutation(capture.as_str());

                overwrite.replace_range(
                    capture.start()..capture.end(),
                    mutation);


                if counter == 0 {break}
                counter-=1;
            }

            *self = overwrite;
            true
        }
        else{
            false
        }

    }
}
