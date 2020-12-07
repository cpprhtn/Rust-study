
/* <ownership 규칙>
- 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
- 한번에 딱 하나의 오너만 존재할 수 있다.
- 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).
*/

fn main() {
    //string type
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.

    println!("{}", s); // `hello, world!`

    //unique_ptr과 같음. 복사시 move.
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);

    //힙 데이터 깊이 복사하기를 원할시 clone 공용 메소드 사용
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //컴파일 타임에 결정되어 있는 크기의 타입은 스택에 모두 저장되기 때문에 복사 O.
    /* <카피 가능한 types>
    - u32와 같은 모든 정수형 타입들
    - true와 false값을 갖는 부울린 타입 bool
    - f64와 같은 모든 부동 소수점 타입들
    - Copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 Copy가 되지만, (i32, String)은 안됩니다.
    */
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    
}
