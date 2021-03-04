use rand::Rng;

fn main() {
    // if
    let a = 1;
    if a > 0 {
        println!("{} greater than 0", a);
    } else if a < 0 {
        println!("{} less than 0", a);
    } else {
        println!("{} equal to 0", a);
    }

    // 三元表达式
    let b = rand::thread_rng().gen_range(1, 101);
    let string = if b > 50 { "大" } else { "小" };
    println!("string: {}", string);

    // loop 循环
    let mut c = 0;
    loop {
        c += 1;
        if c > 5 {
            break;
        }
    }
    c = 0;

    // loop 循环 break 可以返回结果
    let result = loop {
        c += 1;

        if c == 10 {
            break c * 2;
        }
    };
    println!("loop result: {}", result);

    // while 循环
    let mut d = 0;
    while d < 3{
        d +=1;
    };
    println!("while result: {}", d);

    // for 循环
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("for element: {}", element);
    };

    // for range
    for i in (0..3).rev() {
        println!("for range: {}", i);
    }
}
