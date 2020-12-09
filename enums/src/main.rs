
// enum 열거형
fn main() {
    //ver 1
    /*
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    */
    //ver 2
    //데이터 타입을 명시하여 struct를 쓸 필요가 없어짐
    /*enum IpAddr {
        V4(String),
        V6(String),
    }
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    
    let loopback = IpAddr::V6(String::from("::1"));
    */
    //ver 3
    //서로 다른 종류의 열거형 사용
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    
    let loopback = IpAddr::V6(String::from("::1"));

    //
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage; // 유닛 구조체
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // 튜플 구조체
    struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

    impl Message {
        fn call(&self) {
            // 메소드 내용
        }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();

    //
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    //
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y;

    let sum = match y {
        Some(y) => x + y,
        None => x
    };

    println!("x + y = {}", sum);
}
