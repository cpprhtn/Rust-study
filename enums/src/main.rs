
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

    let sum2 = x + y.unwrap();  //패닉

    let sum3 = x + y.unwrap_or(0);  // y가 None이면 x

    let sum4 = y.map(|y| x + y).unwrap_or(x);

    let sum5 = if let Some(y) = y { x + y } else { x };

    let sum6 = add(5, Some(5)).unwrap_or(0);    // y가 None이면 0

    let sum7 = y.map_or(x, |y| x + y);
    
    //let sum8 = Some(x).zip_with(y, |x, y| x + y);

    let sum9 = unsafe {
        let [_, y] = std::mem::transmute::<_, [i8; 2]>(y);
        x + y
    };

    println!("x + y\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", sum, sum2, sum3, sum4, sum5, sum6, sum7, sum9);
}

fn add(x: i8, y: Option<i8>) -> Option<i8> {
    Some(x + y?)
}
