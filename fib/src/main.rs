fn main() {
    for i in (0..20) {
        println!("fib: {}", fib(i));
        println!("fib1: {}", fib1(i));
        println!("fib2: {}", fib2(i));
    }
}

fn fib(round: isize) -> isize {
    let mut a = 0;
    let mut b = 1;

    let mut round = round;

    while round > 0 {
        let c = a + b;
        a = b;
        b = c;
        round -= 1;
    }

    let result = a;
    result
}

fn fib1(n: isize) -> isize {
    let mut a = 0;
    let mut b = 1;
    let mut n = n;

    let result = loop {
        if n == 0 {
            break a;
        }
        let c = a + b;
        a = b;
        b = c;
        n = n - 1;
    };
    result
}


fn fib2(n: isize) -> isize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    };

    fib2(n - 1) + fib2(n - 2)
}
