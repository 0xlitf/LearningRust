
#![allow(unused)]
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';


    let t = true;

    let f: bool = false; // with explicit type annotation

    
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];
    let a = [3, 3, 3, 3, 3];
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }


    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
