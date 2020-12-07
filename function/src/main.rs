fn main() {
    let x = 3;
    another_function(x);

    let y = five();

    println!("The value of y is: {}", y);

    let x = plus_one(x);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {    //함수의 선언부에서, 여러분은 반드시 각 매개변수의 타입을 정의해야 합니다.
    println!("The value of x is: {}", x);
}

fn five() -> i32 {  
//우리는 반환되는 값을 명명해야 할 필요는 없지만, 
//그들의 타입은 화살표(->) 뒤에 선언해야 합니다. Rust에서 반환 값은 함수 본문의 마지막 표현식의 값과 동일합니다. 
    5
}

fn plus_one(x: i32) -> i32 {
//반환하는 값에 대해서는 ";"쓰지 않기!!!
    x + 1
}