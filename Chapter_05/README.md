# Using Structs to Structure Related Data

## Defining and Instantiating Structs

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");
}

```

- struct에 대해서 특이사항은 아직까지 없음
- 모든 필드가 mut이거나 immut 하게 설정할 수 있지만, 일부만 다르게는 불가능함

생성자 함수 만들기

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}

```
- 필드명이랑 파라미터랑 같으면 이렇게 쓸 수 있음

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        // 이것만 나중에 써주면 된다!
        ..user1
    };
}

```

- 소유권 이전은 일어난다

### Using Tuple Structs Without Named Fields to Create Different Types

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

```
튜플과 별 다른 특이점 없음, 같은 구성요소더라도 다른 타입 취급된다는것 외에는

### Unit-Like Structs Without Any Fields
- 특성을 부여하기위해서 사용한다고 하는데, 무슨 의미인지 모르겠다 => 나중에 다룬다고는 한다.
