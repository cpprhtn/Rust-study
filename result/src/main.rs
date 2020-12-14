/*
T는 성공한 경우에 Ok variant 내에 반환될 값의 타입을 나타내고 
E는 실패한 경우에 Err variant 내에 반환될 에러의 타입을 나타내는 것
*/

enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;


fn main() {
    let f = File::open("hello.txt");

    // let f: u32 = File::open("hello.txt");

    // match 표현식을 사용하여 발생 가능한 Result variant들을 처리
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    // expect를 사용하여 result 처리
    let f = File::open("hello.txt").expect("Failed to open hello.txt");


    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    // match를 이용하여 호출 코드 쪽으로 에러를 반환하는 함수
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
    
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    
        let mut s = String::new();
    
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}
