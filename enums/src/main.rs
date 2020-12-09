
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

}
