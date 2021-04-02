fn main() {
    println!("{}", fib(1000));
}


fn fib(n: u16) -> u64 {
    let res = match n {
        0 => 0,
        1 | 2 => 1,
        3 => 2,
        _ => calc_fib(n)
    };
    
    fn calc_fib(a: u16) -> u64 {
        
        let mut x: u64 = 0;
        let mut y: u64 = 1;
        let mut counter: u16 = 0;
        
        let mut z: u64 = 0;
        
        while counter < a - 1 {
            z = x + y;
            x = y;
            y = z;
            counter += 1;
        }
        return z;
    }
    
    res
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
        assert_eq!(fib(50), 12586269025);

    }
}
