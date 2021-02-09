use std::thread;
use std::time::Duration;


// 실행 시간이 2초 걸리는 가상의 계산을 대신하는 함수
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {}
