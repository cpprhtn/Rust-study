fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter(): 각 요소를 반환하는 함수
    // enumerate(): iter의 결과값을 튜플의 일부로 만들어 반환
    for (i, &item) in bytes.iter().enumerate() {
        // 공백일 경우에는 공백 위치 반환
        if item == b' ' {
            return i;
        }
    }

    // 공백이 아닐 경우에는 문자열 길이 반환
    s.len()
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word = 5

    s.clear(); // String 내용을 ""(빈내용)으로 변경

    // 스트링 슬라이스
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    //
    let s = String::from("hello");
    // 인덱스 0부터 시작시에는 범위에 0 생략가능
    let slice = &s[0..2];
    let slice = &s[..2];
    // 인덱스 마지막까지 범위시에는 끝 숫자 생략가능
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    // 전범위
    let slice = &s[..];

    //
    let word2 = first_word_str(&s); // word = 5

    // s의 타입: &str
    let s = "Hello, world!";

    //&String과 &str은 같은의미

    //
    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작
    let word3 = first_word_str(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작
    let word4 = first_word_str(&my_string_literal[..]);

    // 스트링 리터럴 또한 스트링 슬라이스에 포함
    let word5 = first_word_str(my_string_literal);

    println!("1: {}\n2: {}\n3: {}\n4: {}\n5: {}",word, word2,word3, word4, word5);

    //
    let a = [1, 2, 3, 4, 5];
    //이 슬라이스는 &[i32] 타입을 가짐
    let slice = &a[1..3];
}