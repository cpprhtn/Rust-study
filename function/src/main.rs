fn main() {
    let x = 3;
    another_function(x);

    let y = five();

    println!("The value of y is: {}", y);

    let x = plus_one(x);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {    //함수의 선언부에서, 반드시 각 매개변수의 타입을 정의!!!
    println!("The value of x is: {}", x);
}

fn five() -> i32 {  
//반환 값은 함수 본문의 마지막 표현식의 값과 동일
    5
}

fn plus_one(x: i32) -> i32 {
//반환하는 값에 대해서는 ";"쓰지 않기!!!
    x + 1
}