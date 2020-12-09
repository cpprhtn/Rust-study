fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.\n",
        area(length1, width1)
    );

    // 튜플로 처리
    let rect1 = (50, 30);

    println!(
        "The area_2 of the rectangle is {} square pixels.\n",
        area_2(rect1)
    );

    // 구조체 사용
    let rect2 = Rectangle { length: 50, width: 30 };

    println!(
        "The area_3 of the rectangle is {} square pixels.\n",
        area_3(&rect2)
    );

    // 디버깅
    let rect3 = Rectangle { length: 50, width: 30 };

    println!("rect3 is {:?}\n", rect3);
    println!("rect3 is {:#?}\n", rect3);
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

/* 디버깅...
- 구조체 정의부분 바로 전에 #[derive(Debug)] 어노테이션을 추가해주기
- {} 내에 :? 명시자를 집어넣어 println!에게 Debug라 불리우는 출력 포맷을 사용하도록 알림

- :?대신 :#?를 쓰면 읽기에 더 수월한 출력을 보여줌
    - 구조체에 값을 채워서 불러오는듯
*/