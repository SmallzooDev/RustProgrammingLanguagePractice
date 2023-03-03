# Enums and Pattern Matching

- 다른 언어에서의 이넘과는 무슨 차이가 있을까?
- 이넘에 대한 정의를 잘 다루고 있다
  - 모든케이스를 속하게 할 수 있으면서, n개로 구분 할 수 있는 경우

```
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

```

```
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

```

이런게 가능하다고 한다. 내 생각에 이건 유용하고 슥 보고 쓰기는 쉽지만, 왜 되는지를 알아보는게 중요한 공부일 것 같다.

"우리가 정의한 이넘 variant는 해당 이넘의 인스턴스를 생성하는 생성자 역할을 한다."

이런 enum 구조체(인스턴스를 생성하니까?)를 쓰면 그냥 구조체를 쓰는것보다 좋은점이 많은데, 대표적으로 이런것들도 가능

```
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}


```

실제로 사용하는 라이브러리에는 이걸 잘 활용하고 있다고 한다.

```
#![allow(unused)]
fn main() {
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
}

```


```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

```

```
struct QuitMessage;
struct MoveMessage {
  x: i32;
  y: i32;
};
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

```
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

이렇게 보면 이넘이 하나의 인터페이스와 다양한 구현체를 간단하게 처리 할 수 있는 것 같기는 한데,
내가 아는 이넘 개념이랑 너무 혼동이 오기도 하고, 이런 방식의 한계가 궁금하기는 하다.


## Option Enum 예시

옵션 타입은 값이 있거나 없을 때 와 같은 매우 흔한 상황을 encode한 이넘이다.

배열에서 첫 원소를 요청할 때

요소가 있는 배열 -> 첫 요소

요소가 없는 배열 -> 없음

이러한 상황, 개념을 (값이 있을 수 도 있고, 없을 수 도 있다)를 타입시스탬으로 표현한다는것은 컴파일러가 프로그래머가 나타날 수 있는 모든 상황을 다뤘는지를 체크 해 줄 수 있다.

무튼 null을 주제로 이야기 하고 싶어하는건 알았는데, 러스트에는 null이 없다는건 조금 놀라웠다.

Null은 수조원짜리 실수였다는 Tony Hoare의 이야기
그게 궁금해서 검색하다가 레딧에서 이런 댓글을 봤다 ㅋㅋ
Hmm 44 years, so that means Stroustrup's apology should be coming up around 2027.

그래서 검색해봤는데..
- C++ was written by  Bjarne Stroustrup  at Bell Labs during 1983-1985.
- C++ is an extension of C.  
- Prior to 1983, Bjarne Stroustrup added features to C and formed what he called "C with Classes".
- The term C++ was first used in 1983.

학부 수업 들을 때 빼고는 c++을 안써봤는데, 이정도 악명이면 다시 궁금해지기는 한다!

무튼 null의 문제점은 간략히 낫널 밸류에 널을 쓰기 너무 쉽다고 하는데 널이 없는 세상에서 살아보지 못해서 들을 때 마다 와닿지는 않는 것 같다.

그러나 널이 유용할 때 는 '지금' 밸류가 옳지 않거나, 무슨 이유로든 없을 때 를 표현하기에 유용하다고 한다.

결론적으로 러스트는 Option Enum을 쓴다고 합니다. (그냥 옵셔널)

```
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

```
그 이후로는 옵셔널과 똑같은 개념적인 내용이다.

밸류가 있을수도 없을수도 있는 경우는 옵셔널을 반환하게 애초에 되어있으면 무조건 처리를 하게 될 것 이며,

반대의 경우 널을 던지면, 똘똘한 프로그래머만 if not null을 하게 될 것이라는 내용

## The match Control Flow Construct

2장의 예제 코드에서 가장 궁금하던 문법이었다. 굉장히 현대적인 방식의 문법?(예약어?)인 것 같은 느낌이 강하게 들었기 때문이다.

본문에서는 match expression을 동전 분류기로 비유하고 있다, 일단 하나의 구멍으로 다 넣고, 크기가 맞으면 들어가서 분류되고, 더 작은 구멍을 만나고 뭐 이런식으로.(switch..?)

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}

```

패턴에 일치하면 엮여있는 코드를 실행하는 식으로 진행됨
- 주의할점은 패턴과 매칭되어 있는 코드는 표현식이고, 표현식의 값은 전체 match expression의 값이 된다는 것

## Patterns That Bind to Values


```
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

```

struct enum인 경우 이런것도 가능하다는 것!

## Matching with Option<T>

```
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

```


이건 이제 상당히 눈에 잘들어오는 코드가 되었다.

## Matches Are Exhaustive

```
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

```

적당히 타협해서 이렇게도 가능하다!
값을 쓰지 않고 싶을때는 이렇게
```
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

```


```
    let nth_num: u32 = match nth_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please input number");
            return;
        }
    };
```

이 코드도 조금 더 읽히는데 아직 공부가 부족한 부분이 있다.
parse() 의 결과값과 Match를 하는 부분?
u32를 인자로 넘겨받지 않았음, 그럼 parse() 함수는 뒤에 {}를 보는건가? -> 불가능해보임
그게 아니라면, match와 같은 조건과 함께 동작할 수 있는 제약이 있나? -> 블가능해 보임
아니면 지금 정의하는 샘이 되는건가? -> 뭔가 그럴지도?
아니면 가장 가능성 높은 parse()로 반환 가능한 타입 전부를 반환 하고 (Ok(T) 형식으로 .. ), 
사용하는 사람이 알아서 쓰는건가?
TODO : 이거 로그 찍어보기
근데 마지막 생각이 결국 맞는 것 같기는 한데, 확실히 찾아봐야 안까먹을 것 같다.

match예약어에 직관적이지 않은 새로운 배우지 않은 기능이 있다 vs 후자인데 후자가 맞는것같긴하다

## Concise Control Flow with if let

```
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

```

```
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

```

```
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

```


```
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

```