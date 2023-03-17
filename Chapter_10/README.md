# Generic Types, Traits, and lifetime

이 장에 대한 개괄, 그리고 기본적으로 generics를 코드 중복의 해소 관점에서 접근한다.
오랫만에 언어를 다시 배우고 있어서 그런지, 너무 당연하게 쓰고있었는데 당연히 Genrics는 코드 중복을 해소하기 위해서 사용한다는걸 잊어버리고 있었던 것 같다. 그 개념을 잊거나 모른다는게 아니라 generics를 코드 중복의 해소로 단정지어서 이야기하는게 낯설었다가, 다시 생각해보니 코드 중복 말고는 쓸 이유가 없는게 당연하다고 생각이 들었다.

```
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    assert_eq!(*largest, 100);
}

```


```
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

```

```
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 6000);
}

```

딱히 설명 할 필요 없는 코드지만, 이 작업을 세가지로 요약했다.
1. 중복된 부분을 가린다.
2. 중복된 코드를 함수 바디 안에 넣고, input과 return을 특정짓는다
3. 중복된 두 군데에 함수 호출을 대신해서 넣는다.

이와같은 작업을 generics를 통해서 한다고 한다.

## Generic Data Types
우리는 함수 signatures나 구조체들같은 아이탬의 정의를 만들어 내기 위해서 사용한다.
그러면 우리는 그것들을 다양한 구체적인 타입들과 함께 사용 할 두 있다.

### In Function Definitions

```
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}
```

그냥 제네릭에대한 이야기들
구조체에서 타입에 따라 가질 수 있는 메서드를 특정해둘 수 있다.

monomorphism을 통해서 성능상 차이가 없도록 만든다고 하는데,
오히려 반대로 다른 언어들도 당연히 그렇게 하는게 아닌가 알아보고 싶어졌다.
제네릭을 사용한다면, 컴파일 시점에 무조건 타입이 확정되는데 말이다.


알아본 내용 : 
- https://www.programcreek.com/2011/11/what-do-java-objects-look-like-in-memory/
- https://gist.github.com/Kimundi/8391398

자바와 러스트 제네릭의 차이점이지만, 레퍼런스와 메모리 관리와 같은 내용들도 들어있어 너무 읽기 좋았다


## Traits : Defining Shared Behavior

- traits : 특정 타입과 다른 타입이 공유 할 수 있는 기능을 정의하는것
- 특정한 행동을 추상적인 방법으로 공유 할 수 있다.
- trait bound 를통해서 어떤 타입이 공유할지 정의 할 수 있다.

> 다른언어 (아마도 자바)의 interface와 비슷해 보일 수 있으나, 차이점이 있다.



### Trait 정의하기
- 타입의 Behavior는 그 타입에서 부를 수 있는 메소드들로 구성되어있으며, 다른 타입은 같은 Behavior 공유할수있다.

```
pub trait Summary {
    fn summarize(&slef) -> String;
}
```

이제 이 trait를 구현하는 객체들은 이 안에 있는 모든 메서드를 구현해야한다. -> 여기까진 100% 인터페이스

### Implementing a Trait on a Type

```
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

}
```


```
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());

}

```

특이한점은 trait를 별도로 불러와야한다는 점.

### Default Implementations

이렇게 default impl이 가능함
```
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

그리고 이 안에 같은 trait 안에있는 다른 method도 사용 할 수 있음

```
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```
> 이러한 방식의 가장 큰 장점!
> crate를 제공하면서, 엄청 큰 기능을 구현하고 그 일부의 구현만 사용자에게 시킬 수 있음
> 프레임워크화 하기에 매우 유용 할 것 같다

### Traits as Parameters

```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

```

여기도 인터페이스 같긴 하네 ㅋㅋ....
뭔가 훨씬 캐주얼한 자바 객체지향 프로그래밍 같은 느낌이 든다.
생각보다 훨씬 제약이 적고 자유로워 보이지만, 또 빡빡하기도 하고..

### Trait Bound Syntax

사실 위에는 
```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
이 코드의 sugar Syntax 였습니다.

```
pub fn notify(item1: &impl Summary, item2: &impl Summary) 
```
그래서 이런 경우라면 무조건 item1, item2의 타입이 달라야 한다네요

### Specifying Multiple Trait Bounds with the + Syntax

```
// 이런것도 가능하며
pub fn notify(item: &(impl Summary + Display)) {
// 이 코드의 sugar Syntax
pub fn notify<T: Summary + Display>(item: &T) {

```

### Clearer Trait Bounds with where Clauses

너무 많은 trait bounds (trait으로 생성되는 type bound 같은 것으로 생각됨) 를 사용하면 
각각의 제네릭이 각각의 바운드를 가져서멀티 제네릭 타입 파라미터는 너무 많은 트레잇 바운드를 작성해줘야함
이건 함수읽는게 너무 힘들어져서 별도의 문법을 제공한다.

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

```

```
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{

```

```
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

```

리턴문에도 가능, 이정도면 그냥 타입이긴 하다

iterable protocol, closure 같은 것들을 구현하기 쉽다고 하는데, iter는 알겠지만, 클로저는 무슨 관련이 있는지
감이 안잡히긴 한다.

```
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

```
하지만, 리턴을 trait의  구현체중 아무거나 반환하게 할 수 는 없다,
컴파일 시점에서 명시적으로 하나의 구현체만 반환해야 컴파일 에러가 나지 않는다.
interface와의 첫 번째 차이점이 나온 것 같다.

### Using Trait Bounds to conditionally Implement Methods

특정 조건에 따라 메서드를 구현해 둘 수 있다.
```
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

```
impl<T: Display> ToString for T {
    // --snip--
}
```
