// panic! 
/*
일반적으로 panic!이 발생한다면 해당 프로그램의 되감기 실행
하지만 되감기가 된다면 리소스를 많이 잡아먹음
이를 방지하고, 즉시 그만두기를 하기위해서는 Cargo.toml에서 아래 코드를 추가
```rust
[profile.release]
panic = 'abort'
```
*/

fn main() {
    //panic!("crash and burn");

    // 패닉을 일으키는 벡터의 끝을 넘어선 요소에 대한 접근 시도
    let v = vec![1, 2, 3];

    v[99];
}