
// 사용자 계정에 대한 정보를 저장하는 구조체
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 사용자의 이메일과 이름을 받아 User 구조체의 인스턴스를 반환하는 함수
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // 구조체 User의 인스턴스 생성
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // user1의 일부 값들을 재사용하여, 구조체 User의 인스턴스 user2를 새로 생성
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // 입력 받는 이외의 값들은 user1에서 재사용
    };

    // 이름이 없고 필드마다 타입은 다르게 정의 가능한 튜플 구조체
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}