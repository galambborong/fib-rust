use std::io;
use num_format::{Locale, ToFormattedString};


fn main() {
    println!("Find out the nth Fibonacci number.\nEnter your number:");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input: u16 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let result = fib(input).to_formatted_string(&Locale::en);

        println!("The {}th fibonacci number is {}", input, result);
        break;
    }
}

fn fib(n: u16) -> u128 {
    let res = match n {
        0 => 0,
        1 | 2 => 1,
        3 => 2,
        _ => calc_fib(n),
    };
    res
}

fn calc_fib(a: u16) -> u128 {
    let mut x: u128 = 0;
    let mut y: u128 = 1;
    let mut counter: u16 = 0;

    let mut z: u128 = 0;

    while counter < a - 1 {
        z = x + y;
        x = y;
        y = z;
        counter += 1;
    }
    z
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn fib_less_than_10() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(8), 21);
        assert_eq!(fib(9), 34);
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn fib_bigger_n() {
        assert_eq!(fib(50), 12_586_269_025);
        assert_eq!(fib(100), 354_224_848_179_261_915_075);
    }

    #[test]
    fn test_calc_fib() {
        assert_eq!(calc_fib(10),55);
        assert_eq!(calc_fib(20),6_765);
    }
}
