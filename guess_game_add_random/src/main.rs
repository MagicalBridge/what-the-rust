use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("猜数游戏，包含随机数！");

    let secret_number = rand::thread_rng().gen_range(1..101);


    println!("神秘数字是:{}",secret_number);

    println!("猜测一个数");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行信息");

    let guess_to_string:u32 = guess.trim().parse().expect("please type a number!");

    match guess_to_string.cmp(&secret_number) {
        Ordering::Less => println!("Too small!!"),
        Ordering::Greater => println!("Too big!!"),
        Ordering::Equal => println!("You win!!"),
    }

    println!("你猜测的数字是：{}",guess)
}

