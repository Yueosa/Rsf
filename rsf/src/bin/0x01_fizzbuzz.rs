fn main() {
    println!("=== if ===");
    iffn();
    println!("=== match ===");
    matchfn();
}

fn iffn() -> () {
    for i in 1..=105 {
        let mut res: String = String::new();
        
        if i % 3 == 0 {
            res.push_str("Fizz");
        }
        
        if i % 5 == 0 {
            res.push_str("Buzz");
        }
        
        if i % 7 == 0 {
            res.push_str("Bang");
        }
        
        println!("{}", if res.is_empty() { i.to_string() } else { res });
    }

}



fn matchfn() -> () {
    for i in 1..=105 {
        match ( i % 3 == 0, i % 5 == 0, i % 7 == 0 ) {
            (true, true, true) => println!("FizzBuzzBang"),
            (true, true, false) => println!("FizzBuzz"),
            (true, false, true) => println!("FizzBang"),
            (false, true, true) => println!("BuzzBang"),
            (true, false, false) => println!("Fizz"),
            (false, true, false) => println!("Buzz"),
            (false, false, true) => println!("Bang"),
            _ => println!("{i}")
        }
    }
}

