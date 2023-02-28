use std::io;

fn main() {
    let mut nth_num = String::new();
    println!("please input nth number on fibo");

    io::stdin()
        .read_line(&mut nth_num)
        .expect("Failed to read line");

    let nth_num: u32 = match nth_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please input number");
            return;
        }
    };

    println!(
        "Your input was {nth_num}, and {nth_num}th fibo num is {}",
        fibo(nth_num)
    );
}

fn fibo(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    fibo(num - 1) + fibo(num - 2)
}
