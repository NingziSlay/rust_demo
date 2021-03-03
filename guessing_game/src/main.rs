use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    println!("start guessing game, secret_number is between 0 and 100");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("secret number is {}", secret_number);

    loop {
        let mut guess = String::new();

        println!("input a number");

        io::stdin().read_line(&mut guess).expect("read error");

        let guess: u32 = match guess.trim().parse(){
            Ok(x) => x,
            Err(_) => {
                println!("please input a real number which is greater than 0");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less than the secret_number"),
            Ordering::Greater => println!("greater than the secret_number"),
            Ordering::Equal => {
                println!("bingo!");
                break;
            }
        }
    }
}
