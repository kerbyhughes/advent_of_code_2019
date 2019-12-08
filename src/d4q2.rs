// day 4 question 2

pub fn process() -> Result<i32, &'static str> {
    let mut valid_passwords = 0;
  
    for passwd in 172851..675869 {
        if has_adjacent(passwd) && is_monotonic_inc(passwd) {
            valid_passwords += 1;      
        }
    }
   
    /*
    let test = 123444;
    println!("has_adjacent {}", has_adjacent(test));
    println!("is_mono {}", is_monotonic_inc(test));
    */

    Ok(valid_passwords)
}

fn has_adjacent(passwd: u32) -> bool {
    let digits: Vec<_> = passwd.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut i = 0;
    while i < digits.len() - 1 {
        if digits[i] == digits[i + 1] {
            if only_two_adjacent(passwd, digits[i]) {
                return true;
            }
        }

        i += 1;
    }
    
    return false;
}

fn only_two_adjacent(passwd: u32, num: u32)-> bool {
    let digits: Vec<_> = passwd.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut i = 1;
    while i < digits.len() - 1 {
        if digits[i - 1] == num && num == digits[i + 1] {
           return false;
        }

        i += 1;
    }

    return true;
}

fn is_monotonic_inc(passwd: u32) -> bool {
    let digits: Vec<_> = passwd.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut last = 0;
    for d in digits {
        if d < last {
            return false;
        }

        last = d;
    }

    return true;
}

