/* <ownership 규칙>
러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
한번에 딱 하나의 오너만 존재할 수 있다.
오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).
*/

fn main() {
    //string type
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.

    println!("{}", s); // `hello, world!`
}
