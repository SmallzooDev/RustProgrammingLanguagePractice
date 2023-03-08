# Error Handling

- 에러 처리는 프로그래밍의 일부
- 대부분의 언어는 에러를 exception을 통해 처리한다.
- 하지만 회복 가능 여부에서 에러를 나누어야 정확한 관리가 된다고 한다.
- Result<T, E> 에서의 회복 가능한 에러와
- panic! 메크로에서의 회복 불가능한 에러에 대해서 다룬다.



## Unrecoverable Errors with panic!

- 패닉은 우리가 의도해서 일으키거나, 우리가 의도하지 않았을 때 일어나는 두가지가 있고, 두가지 모두 프로그램은 멈춘다.
- 패닉이 일어났을 때 프로그램이 멈추며, 스택 트레이스를 찍는데 그게 싫으면 운영체제가 메모리를 회수하도록 하고, 한 큐에 종료도 가능

## Using a panic! Backtrace

- 패닉이 어디서 일어났는지 스택을 찍어줌, 다른언어와 같음


## Recoverable Errors with Result
- 많이 봐왔던 것

```
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

```
```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}


```
이런 것도 당연히 가능 

## Shortcuts for Panic on Error: unwrap and expect

```
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

```

- 조금 더 편하게 에러를 핸들하도록, unwrap과 expect가 있다. expect는 메세지를 넣을 수 있어 더 자주 사용


## Propagating Errors
- 우리가 작성하는 함수도 result를 사용해서 에러를 전파 할 수 있음
```
#![allow(unused)]
fn main() {
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
}

```

이런 방식은 너무 흔해서 ? 연산자가 있음

```
#![allow(unused)]
fn main() {
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
}

```

- Result값 뒤의 ? 연산자가 하는일 : OK면 안에 있는 밸류를 전해주고, ERR라면 에러를 전파한다.
- ? 연산자는 from trait을 불러줌 -> 반환하는 에러 형태를 일치시켜줄 수 있다.
- 자동으로 from을 호출해서 함수 리턴값에 해당하는 에러로 변환시켜주는 코드이긴 하지만 자세한 설명이 없긴 하다.
TODO: 검색해보기

```
#![allow(unused)]
fn main() {
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
}

```
일단은 ? 연산자로 이만큼 보일러 플레이트를 없앨 수 있다.

```
#![allow(unused)]
fn main() {
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
}

```


이것도 가능하다고는 하는데, 주제와 많이 연관되어 있지는 않은 것 같고, 이렇게까지 할 것 같지는 않다, 심지어 에러메세지도 못쓴다.


## To panic! or Not to panic!

### Cases in Which You Have More Information Than the Compiler

```
fn main() {
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

```
- 이건 확실히 유효한 ip이고 그걸 우리는 알지만, 컴파일러는 그걸 모르기 때문에 parse의 예외처리를 강요한다.
- 이런경우 이렇게 expect를 이용 할 수 있다.

### Guidelines for Error Handling
- 나쁜 상태에 빠질 것 같으면 패닉을 호출해라
- 나쁜 상태 : 보통 unexpected한 것 (유저가 뭔가를 입력한다던지 하는 예상가능한 상태가 아닌 것)
- 말이 안되는 리턴을 받았다면 보안적인 이유로라도 프로그램을 멈추는 것이 나음
- 대표적으로 다른 사람의 코드를 사용해서 리턴을 받는데, 그 과정을 관여 할 수 없으면 무조건 panic이 맞다
- 모든 케이스를 점검 하기는 번거로워서 러스트의 타입 시스탬을 이용 할 수있다.

```
#![allow(unused)]
fn main() {
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
}


```

이러면 게스타입을 만들때의 예외처리를 강요받는건가? 아니면 패닉이 나와서 멈추는건가 테스트해보기
