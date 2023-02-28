use std::collections::HashMap;

fn main() {
    let mut map: HashMap<usize, &str> = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");
    map.insert(5, "five");
    map.insert(6, "six");
    map.insert(7, "seven");
    map.insert(8, "eight");
    map.insert(9, "nine");
    map.insert(10, "ten");
    map.insert(11, "eleven");
    map.insert(12, "twelve");
    map.insert(13, "thirteen");
    map.insert(14, "fourteen");
    map.insert(15, "fifteen");
    map.insert(16, "sixteen");
    map.insert(17, "seventeen");
    map.insert(18, "eighteen");
    map.insert(19, "nineteen");
    map.insert(20, "twenty");
    map.insert(30, "thirty");
    map.insert(40, "forty");
    map.insert(50, "fifty");
    map.insert(60, "sixty");
    map.insert(70, "seventy");
    map.insert(80, "eighty");
    map.insert(90, "ninety");
    map.insert(100, "hundred");
    map.insert(1000, "thousand");

    let mut sum = 0usize;
    for i in 1..1001 {
        match map.get(&i) {
            Some(x) => sum += x.len(),
            None => sum += calculate(&map, i as usize),
        }
        if i == 1000 || i == 100 {
            sum += 3; // one
        }
    }
    println!("{}", sum);
}

fn calculate(map : &HashMap<usize, &str>, i : usize) -> usize{
    let mut sum = 0usize;

    if i.to_string().len() == 3 {
        sum += map.get(&(i / 100)).unwrap().len() + map.get(&100).unwrap().len();
    }

    let second = i.to_string().chars().nth(1) == Some('0');
    let first = i.to_string().chars().nth(2) == Some('0');
    if !second || !first {
        if i.to_string().len() == 3 {
            sum += 3; // and
        }
        match (second, first) {
            (true, false) => {
                let one_digit = i % 10;
                match map.get(&one_digit) {
                    Some(x) => sum += x.len(),
                    None => (),
                }
            }
            (false,true) => {
                let two_digit = (i % 100).to_string().chars().nth(0).unwrap().to_digit(10).unwrap() as usize * 10;
                match map.get(&two_digit) {
                    Some(x) => sum += x.len(),
                    None => (),
                }
            }
            (false,false) => {
                let one_two_digit = i % 100;
                let two_digit = (i % 100).to_string().chars().nth(0).unwrap().to_digit(10).unwrap() as usize * 10;
                let one_digit = i % 10;
                if map.get(&one_two_digit).is_some() {
                    sum += map.get(&one_two_digit).unwrap().len();
                } else {
                    match map.get(&two_digit) {
                        Some(x) => sum += x.len(),
                        None => (),
                    }
                    match map.get(&one_digit) {
                        Some(x) => sum += x.len(),
                        None => (),
                    }
                }
            }
            (_,_) => (),
        }
    }
    sum
}
