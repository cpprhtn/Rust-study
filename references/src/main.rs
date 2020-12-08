
/* <참조자의 규칙>
- 어떠한 경우이든 간에, 여러분은 아래 둘 다는 아니고 둘 중 하나만 가질 수 있습니다:
    - 하나의 가변 참조자
    - 임의 개수의 불변 참조자들

- 참조자는 항상 유효해야만 한다.
*/

fn main() {
    //
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //
    change(&mut s1);

    //
    let reference_to_nothing = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change (s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}