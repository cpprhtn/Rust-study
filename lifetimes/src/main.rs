

// 라이프타임은 댕글링 참조자를 방지


// 함수에서의 제네릭 라이프타임
// 두 스트링 슬라이스 중 긴 쪽을 찾기 위해 longest 함수를 호출하는 main 함수
fn main() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
