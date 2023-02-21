fn main() {
    
    let mut pow = 2u128.pow(100);
    let mut digit = Vec::new();
    while pow > 0 {
        digit.push(pow % 10);
        pow /= 10;
    }

    for i in 0..900 {
        let mut for_the_next = 0;
        for j in digit.iter_mut() {
            *j *= 2;
            *j += for_the_next;
            if *j > 9 {
                for_the_next = *j / 10;
                *j %= 10;
            } else {
                for_the_next = 0;
            }
        }
        if for_the_next > 0 {
            digit.push(for_the_next);
            for_the_next = 0;
        }
    }
    
    let tot = digit.iter().map(|c| c).sum::<u128>();
    println!("{digit:?}");
    println!("{tot}");
}
