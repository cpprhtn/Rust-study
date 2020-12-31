use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // 쿼리와 파일이름 인자를 보관하는 두 변수 생성
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // 두 번째 인자로 특정된 파일의 내용 읽어들이기
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
