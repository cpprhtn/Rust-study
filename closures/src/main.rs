use std::thread;
use std::time::Duration;


// 실행 시간이 2초 걸리는 가상의 계산을 대신하는 함수
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {}

// 사용자 입력과 임의의 숫자 생성을 시뮬레이션 하기 위한 main 함수와 하드코딩된 값
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
