use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input your quess.");

         // 변수 선언의 기본은 immutable, mutable 하게 만들고 싶으면 mut 예약어 사용해야한다
        let mut guess = String::new();

        // std :: io :: stdin() - 인스턴스 반환
        // read_line() 메서드 사용
        // err, ok enum을 반환하기에 그에 맞는 처리를 해줘야함
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {guess}");

        // 1. use statement로 Ordering Enum 을 가지고 온다
        // 2. cmp 메서드는 전달받은 secret_number와 비교해서 Ordering을 반환해준다.
        // 3. match 를 통해서 cmp 메서드가 반환한 Ordering값에 따라 뭘 해줄지 결정해준다?
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
