pub fn celsius2farenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

pub fn farenheit2celsius(farenheit: i32) -> i32 {
    (farenheit - 32) * 5 / 9
}

pub fn fibonacci_loop(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

pub fn fibonacci_rec(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
    }
}

/*
assert_eq!(calculator::celsius2farenheit(30), 86);
assert_eq!(calculator::farenheit2celsius(86), 30);
assert_eq!(calculator::fibonacci_loop(3), 2);
assert_eq!(calculator::fibonacci_rec(3), 2);
*/
