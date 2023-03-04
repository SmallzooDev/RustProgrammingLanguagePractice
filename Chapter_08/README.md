# Common Collections

배우는건 vector, string, hashmap!

## Storing Lists of Values with Vectors

```
let v:Vec<i32> = Vec::new();
```
```
let v = Vec![1,2,3];
```

푸시하는 경우

```
fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

```


```
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

```


```
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

```

```
fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

```


## Using an Enum to Store Multiple Types

```
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}


```
이게 가능은 하지만, 또 파이썬이나 자스같이 저장하는게 아니라 진짜 배열처럼 시리얼하게 저장한다고함.

## Storing UTF-8 Encoded Text with Strings

다른 언어와는 다르게 스트링이 조금 복잡하고 애먹이는데, 세가지 이유가 있다고 한다.
1. 에러상황에 대해서 더 빡빡하게 군다.
2. 그럼으로 오히려 조금 더 복잡한 ㅏㅈ료구조가 된다.
3. UTF-8 issue

러스트의 스트링이란
1. stl의 String
2. str 

### Creating New String
- 스트링은 벡터의 래퍼 (추가적인 개런티, 규제, 기능을 가진)이기 때문에, 벡터 인스턴스를 만드는것처럼 맹글 수 있다.

```
let mut s = String::new();
```

```
let data = "initial contents";

let s = data.to_strings();

// display 특성을 구현해놓은 것들은 to_string() 메서드를 사용 가능
// 자스의 이터러블 프로토콜이나, 자바의 toString과 비슷 
//the method also works on a literal directly:
let s = "initial contents'.to_string();

```

+로 합칠 수 있기는 하지만

```
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

```

오너쉽 이전도 일어난다.

```
fn add(self, s: &str) -> String {

```


인덱스는 지원하지 않는다!
이유 => 진짜 시리얼한 선형 자료구조 배열로 문자열을 다루는데
utf-8 사용하면서 문자마다 사이즈가 다르다,
char 혹은 bytecode로 변경해서 받아야 한다고 한다.

## Hashmap

```
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

```

별다를게 없어서 오너쉽만 주의

```
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

```


```
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

```

이런게 된다는데 당연히 직관적이라 이해는 쉬우나 어떻게 구현되어있는지 궁금하다.
있다 없다를 enum으로 반환한다는데 그러면 오너쉽을 가져가서 scores도 같이 반환하는건지
내부 아니면 레퍼런스를 가져가서 레퍼런스를 반환하는건지

or인서트는 어떻게 두개의 파라미터를 받는건지 궁금하다
Entry::true(scores) Entry::false(scores)
Entry true {
  scores,
}

implement or_insert(num: u32) {
  
}

이렇게 구현되어있으려나..