# Chapter 4 Ownership

## 느낀점
- 발상의 전환이 곧곧에 보인다. 특히 디폴트로 된 부분이 그렇다 
- 함수 등에서 기본적으로 오너쉽을 넘기고 필요한경우 레퍼런스를 넘기고 필요한경우 변경 권한을 가진 레퍼런스를 넘긴다. 
- 뭔가 mut/immut 때처럼 디폴트가 반대로 된 느낌이라, 반대로 쓰면서 귀찮을 수도 있지만 내가 뭘 하고 있는지 인지시켜주는 느낌이다.
- 이러한 반대의 로직은 오너쉽이라는 개념을 인지시키고 활용하는걸 디자인적으로 잘 해결 한 것 같다
- 스코프가 끝나면 드랍되면서 해제되는 메모리/ 레퍼런스의 전달과 그걸 디자인적으로 기존과 반전시켜 인지시키는 느낌이다.
- 귀찮아 보이지만 의도치 않게 동작하는 것에 있어서 어디를 수정해야 할 지 잘 보일것 같은 느낌도 든다.


## 00. What is Ownership

- 다른 언어에서의 가비지 컬렉션, 메모리 할당/해제 처럼 러스트의 메모리를 관리하는 방법
- 새로운 개념

### 0. Stack & Heap
- 시스템 언어에서는 값이 스택에 있는지 힙에 있는지 여부가 중요
- 오너쉽도 이것과 연관되어 있기에 먼저 다룬다고 적혀져 있다.

stack은 자료구조, heap은 자료구조의 힙과는 다른 개념으로 소개되어있어 조금 모호했지만, 
아래의 링크대로 정리하면 될 것 같다.
https://www.geeksforgeeks.org/stack-vs-heap-memory-allocation/


### Ownership Rules

Ownership 룰
1. 러스트의 모든 값들은 오너가 있다.
2. 오너는 (동시에) 딱 하나만 존재 할 수 있다.
3. 오너가 스코프 밖으로 나가는 경우 값은 dropped 된다.


### Variable Scope
```
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```
- 변수의 스코프는 별다를 것 없다.

### String Type

- 3장에서 다룬 데이터들은 크기를 알아서 스택에 간편하게 저장하고, 필요 없으면 간단하게 pop으로 드랍해버릴 수 있다.
- 하지만 러스트가 메모리를 어떻게 관리하는지를 보기 위해서는 heap에 저장되는 데이터가 필요하며, string이 적절 할 것

- 문자열은 편리하지만 변경 불가능하고, 유저 인풋을 받는 등의 상황처럼 코딩을 하는 시점에는 그 문자열이 어떤 문자열인지 모를 수 도 있다.

(string literal과 String타입을 다르게 써서 헷갈리지만 무튼) 러스트는 String 타입이 있다고 한다.
- 컴파일 시점에는 모르는, 가변길이 문자열을 힙에 저장하는 데이터타입

```
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

```

point는 스트링 리터럴과 스트링 타입의 차이점

### Memory And Allocation
point는 스트링 리터럴과 스트링 타입의 차이점
1. 메모리는 Allocator에 런타임에 메모리를 요청해야함
2. 우리가 String을 다 쓰면 메모리를 반환해야함 

1번은 String::from을 통해서 런타임에 진행됨
2번은 가비지컬렉터가 있다면 해주고, 그게 아니라면 메모리 해제를 코드로 명시해줘야함

2번 방법에서의 고질적인 문제점
1. 까먹으면 메모리를 낭비한다.
2. 너무 일찍 해제하면, 버그가 생긴다.
3. 두번 해제해도 버그

결론적으로 "정확한 시점에 단 한번씩의 메모리 할당과 해제가 필요" -> 딱봐도 어렵다.

러스트의 해결책 -> "메모리는 자동으로 단 한 번, 메모리를 Own 하고있는 변수가 스코프 밖으로 나갈 때 해제된다"

```
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
}
```

** 지금은 매우 간단해보이는 해결책이지만, 이러한 방식을 채택함으로써 코드를 작성하는 방법에 매우 큰 영향을 미친다. 힙에 할당해놓은 다수의 변수를 사용하는 상황을 생각해보면 알 수 있다고 한다. **

### Variables and Data Interaction with Move

```
fn main() {
    let x = 5;
    let y = x;
}

```
고정된 사이즈의 데이터 -> 스택에 담기에 우리가 생각하는대로 동작
x에 저장된 데이터 5가 카피됨

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```

이건 조금 다름
s1에 실제 저장되는 데이터
on Stack  :
{
  ptr: heap에 저장된 문자열의 첫 인덱스
  len: (지금 사용하는)길이
  capacity: (총)길이
}

on heap : 
h e l l o

이렇게 나눠서 저장하고, 위와 같은 코드에서는 카피한다면 스택에 저장한 데이터를 카피함
당연히 비용문제 때문이다.

그런데 만약 s1, s2 둘 다 스코프 밖으로 나간다면? 
(double free error)

그래서 러스트는 위와 같은 상황에서 s1을 더이상 유효하지 않다고 처리해버림

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

결론적으로 얕은 복사는 없고, 깊은복사는 automatically 발생하지 않는다.
우리가 해오던 얕은 복사와 비슷한것들은 그냥 스택부의 복사와 기존 (스택 부분의)데이터를 무효화 시킴으로써, move의 개념이 된다.

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

```

이건 당연히 가능하지만, 이렇게 코드를 작성하게 함으로써 비싼 비용이 든다는걸 인지시키는 맥락

### Ownership and Functions

사실상 변수와 똑같이 동작한다.
```

fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

```


```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

```

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

```
여기서 문제는 함수가 데이터를 사용하려면 오너쉽을 이전받아야하고,
그로인해서 s1은 무효화 되기 때문에
위의 튜플로 다시 변수를 만들어주는 번거로운 일을 해야하긴 한다.


🤔 : 오너쉽을 취하지 않은 데이터를 함수가 조작할 수 없기 때문에 부수효과는 매우 많이 줄어들 것 같고,
(방법은 있다고 하니 더더욱)
스코프나 namespace도 매우 많이 단순해질 것 같다. 
(하지만 바로 reference 나옴...)

### Reference & Burrowing

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

```

바로 위의 문제는 해결됨
그리고 오너쉽이 아니고 burrrowing이라고 하면서

```
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

```

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error

```

디폴트로는 
레퍼런스만 가지고 수정하는 것을 막아두었다.

### Mutable References
```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

```
이렇게 mutable referenece를 둬야 하는데 제한이 있다.

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```
무조건 하나의 mutable reference만 존재 할 수 있다. 즉 버로우는 동시에 하나만!
이건 매우 짜증나기는 하지만, Data Race를 막아 줄 수 있다.
심지어 컴파일에러로 잡혀서 컴파일 시점에 동시성 문제를 예방할 수 있다.

```
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error

```


```

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}


```
이렇게 하면 가능하다고 하는데, 그 이유는 레퍼런스의 스코프는 그것이 소개된 시점부터 마지막으로 사용된 시점 까지라고 한다.

그래서 실행해본 테스트 (진짜 안됨)

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    println!("{} and {}", r1, r2); //test again!

```

(진짜 안될 때 에러인데 레퍼런스의 스코프에 대해서 진짜 자세하게 나온다.)
```
  --> src/main.rs:9:14
   |
4  |     let r1 = &s; // no problem
   |              -- immutable borrow occurs here
...
9  |     let r3 = &mut s; // no problem
   |              ^^^^^^ mutable borrow occurs here
...
12 |     println!("{} and {}", r1, r2); //test again!
   |                           -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `refScopeTest` due to previous error
```
### Dangilng References 

- 사실상 있을 수 없음
- 실제로 허상 포인터를 반환하려고 하면 컴파일 에러가 잡힘

### The Slice Type

string과 &str의 차이점만 알아보면 다 따라오는 내용일 것 같다.

string : 동적 힙 문자열 타입,
str : 불변 utf-8 바이트 동적 길이 문자열

```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

```