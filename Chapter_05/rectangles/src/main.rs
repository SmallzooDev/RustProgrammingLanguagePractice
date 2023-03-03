#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // 1. 필드와 동명의 메서드 가능
    // 2. self에는 &self, &mut self 가능, 그냥 self 도 가능하지만, 오너쉽을 가져가기에 특수한 상황 아니고서는 사용하지않음
    // 3. 접근제어자도 있어서 게터 메서드도 쓴다고 함, 접근제어자는 나중에
    // 4. self를 구분해서 받기 때문에 c++에서 포인터의 메서드 접근 연산자 (->)같은것은 사용 안해도 된다고 함
    // 4. 아마도 타입처럼 지정하고, 메서드 파라미터에서 self를 구분하기 때문에, 어떤 파라미터를 self로 넘길지 이미 지정하기 때문에, 명시적으로 어떤 메서드 호출인지 알 수 있기 때문인 듯 하다
}
