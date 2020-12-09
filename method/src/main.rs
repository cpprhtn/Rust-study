
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

// Rectangle의 내용 안에 함수를 정의하기 위해서, impl (구현: implementation) 블록을 생성 
// self 파라미터를 갖지 않는 함수도 impl 내에 정의하는 것이 허용
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        //self.length > other.length && self.width > other.width
        self.area() > other.area()
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 }; // 1500

    rect1.area(); // 1500

    //
    let rect2 = Rectangle { length: 40, width: 10 }; // 400
    let rect3 = Rectangle { length: 45, width: 60 }; // 2700

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}