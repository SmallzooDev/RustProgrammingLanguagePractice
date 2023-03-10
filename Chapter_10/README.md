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

