use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number");

    let secret_number = rand::rng().random_range(1..=100);
    println!("the secret number is: {secret_number}");

    loop {
        println!("input the number:");
        let mut guess = String::new(); // 关联函数(associated function)，关联函数是类型本身所实现的一种函数
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
