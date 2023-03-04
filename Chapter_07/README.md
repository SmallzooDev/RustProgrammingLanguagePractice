# Managing Growing Projects with Packages, Crates, and Modules

## Packages and Crates

### Crates
- 러스트 컴파일러가 동시에 고려할 수 있는 가장 작은 코드 양의 단위.
- 카르고를 쓰지 않고 rustc를 쓴다면, 러스트는 그 하나의 파일을 크래이트로 여긴다.
- 모듈은 사용 할 수 있다.

두가지로 나뉜다,
1. Binary Crate
   - 실행 가능하도록 컴파일되는 crate
   - 실행 됐을 때 뭘 해야할지 적혀있는 main팜수를 정의해야한다.
   - 모든 cra

## Packages and Crates

### Crates
- 러스트 컴파일러가 동시에 고려할 수 있는 가장 작은 코드 양의 단위.
- 카르고를 쓰지 않고 rustc를 쓴다면, 러스트는 그 하나의 파일을 크래이트로 여긴다.
- 모듈은 사용 할 수 있다.

두가지로 나뉜다,
1. Binary Crate
   - 실행 가능하도록 컴파일되는 crate
   - 실행 됐을  때 뭘 해야할지 적혀있는 main 함수를 정의해야한다.


2. Library Crate
    - main함수가 없으며, executable을 포함하지않는다.
    - 대신 다른 다수의 프로젝트에서 사용 가능한 코드들을 포함한다.
    - 일반적으로 crates란 후자를 의미하는 경우가 많으며, 보통 라이브러리와 같은 의미로 통용된다.

## Defining Modules to Control Scope and Privacy

## Module Cheat Sheet

1. Starting from the crate root : 크래이트를 컴파일 할 때, crate root를 본다.(주로 src/lib.rs or src/main.rs)

2. Declaring modules : 크레이트 루트 파일에서, 모듈을 새로 생성 할 수 있다. mod 예약어와 함께 선언하고, 컴파일러는 다음과 같은 규칙으로 해당 모듈을 찾는다.
  - 다음줄에 컬리브라켓이 있고, 바로 선언이 되어 있는지
  - src/garden.rs
  - src/garden/mod.rs 

3. crate root가 아닌 다른 파일에서 다시 서브모듈을 선언 할 수 있다. 예를들어 서브모듈이 mod vegetables라면,
  - 다음줄에 컬리브라켓이 있고, 바로 선언이 되어 있는지
  - src/garden/vegetables.rs
  - src/garden/vegetables/mod.rs

4. Path to code in modules : 모듈이 있다면, 같은 크레이트 내에 어디에서나 해당 모듈을 참조 할 수 있다. 프라이버시 룰이 허용하는 한 패스를 줘서 불러 올 수 있다.
  - crate::garden::vegetables::Asparagus

5. private vs public : 디폴트로는 프라이빗 모듈이며, pub키워드를 써줘야 public 모듈이 된다.

6. The use keyword : 매번 crate::garden::vegetables::Asparagus 와 같은 키워드를 사용할 필요 없이, use ~ 와 같이 사용할 수 있다.

모듈은 구현을 숨겨서 자식 모듈들은 디폴트로 private이다.
심지어 모듈을 pub으로 해줘도, 부모 모듈이 자식모듈을 언급할 수 있게 할 뿐, 내부 코드에 대한 접근까지 허용한것은 아니다.

```
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

```
이런것도 가능함.

## Making Structs and Enum Public

구조체랑 이넘을 퍼블릭하게 만들 수 있기는 하지만, 또 그 내부 필드들도 퍼블릭이 되는건 아님.


```
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}



```


```
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

```

enum은 필드 디폴트가 pub! (enum 이 pub이라면)

## Bringing Paths into Scope with the use keyword

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

```

use 키워드로 간단하게 줄일수 있지만,

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```
모듈같이 스코프 구분이 달라지는 경우에 주의 할 것

관용적으로 함수를 use로 import 할 때는
```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}

```
부모 모듈까지

구조체인경우는 
```
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

```
해당 구조체만 가져오는 식으로 코드를 작성한다고함

```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}


```

이렇게 as 키워드를 이용해서 작성하기도 함

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

```

이렇게 re export를 하기도 함
이런 경우는  우리의 코드가 패키지 혹은 라이브러리 형태로 제공이 될 때
클라이언트는 내부 구현이 백과 프론트로 나뉘어서 있다고 생각 할 필요가 없음.
결론적으로 그냥 하나의 모듈에서 백/프론트 로직들을 한 겹 더 윗계층에서 처리해주면 깔끔함

## Using External Packages

- cargo.toml파일에 패키지 추가
- 카르고가 적혀있는 패키지(와 의존관계가 있는 다른 패키지들)을 다운로드함

## 마지막으로 이런게 된다는 내용

```
use std::io;
use std::io::Write;

use std::io::{self, Write};

use std::io::{self,W
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
      // --snip--
          Ok(())
}

fn function2() -> IoResult<()> {
      // --snip--
          Ok(())
}


```

이렇게 as 키워드를 이용해서 작성하기도 함

```
mod front_of_house {
      pub mod hosting {
                pub fn add_to_waitlist() {}
                    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
}

```

이렇게 re export를 하기도 함
이런 경우는  우리의 코드가 패키지 혹은 라이브러리 형태로 제공이 될 때
클라이언트는 내부 구현이 백과 프론트로 나뉘어서 있다고 생각 할 필요가 없음.
결론적으로 그냥 하나의 모듈에서 백/프론트 로직들을 한 겹 더 윗계층에서 처리해주면 깔끔함

## Using External Packages

- cargo.toml파일에 패키지 추가
- 카르고가 적혀있는 패키지(와 의존관계가 있는 다른 패키지들)을 다운로드함

## 마지막으로 이런게 된다는 내용

```
use std::io;
use std::io::Write;

use std::io::{self, Write};

use std::io::{self,Wite};

use std::collections::*;

```

## Separating Modules into Different Files

