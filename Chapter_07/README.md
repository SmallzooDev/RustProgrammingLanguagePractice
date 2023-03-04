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

