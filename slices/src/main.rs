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

    //공백이 아닐 경우에는 문자열 길이 반환
    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word = 5

    s.clear(); // String 내용을 ""(빈내용)으로 변경

    //스트링 슬라이스
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}