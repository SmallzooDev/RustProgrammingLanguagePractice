# Variables


1. 디폴트가 immutable 이고 굳이 mutable하게 만들고 싶으면 mut 예약어를 쓰게 만드는 이유
그게 더 안전하고, 에러를 찾기 쉬워서

2. constants vs immutable Variables
- 디폴트로 immutable 한 정도가 아니라 always immutable 하다
- 전역을 비록한 어디에나 선언 가능
- 런타임에 계산된 값을 통해서는 할당이 불가능하다. 오직 constants expression을 통해서만 설정 가능
- 스코프 불문 어디에서나 참조 가능!

```
// 이정도 간단한 계산은 constants expression에 포함인 듯 하다
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

3. Shadowing 같은 이름으로 변수 선언 가능
- 이미 있는 변수의 이름으로 변수를 선언한경우가 shadowing
- 사실은 그 변수가 shadowed 되거나, 스코프가 끝나는 동안 다 덮어쓰는 것

4. Shadowing은 mutable과는 다름 
- let 키워드 없이 사용하면 에러가 나게 된다.
- 몇가지 전환 작업을 한 이후 다시 immutable하게 된다
- 타입의 변수를 shadowing 하는 개념이기 때문에, 당연히 타입도 무관함


