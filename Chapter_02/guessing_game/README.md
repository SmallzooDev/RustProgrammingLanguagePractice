# Programming a Guessing Game

0. import 쪽은귀찮은 문법이 많을 것 같다.

1. match 문법이 신기하다

```
match {val : 아마도 enum?} {
    key (enum) : val (things to do),
    ...
}
```
2. arm Pattern이 뭘 의미하는지는 검색해도 많이 나오지는 않는다.

3. 여기 코드가 너무 마음에 든다.
```
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```
변수 타입지정, parse(), 결과값에 대한 처리 흐름으로 되어있어서 익숙해지면 좋을 것 같다.

4. mut, shadowing 등 재미있는 것들이 많은 것 같은데 잘 파악해둬야겠다.
