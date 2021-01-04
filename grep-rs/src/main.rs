use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    /*
    // 쿼리와 파일이름 인자를 보관하는 두 변수 생성
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // 두 번째 인자로 특정된 파일의 내용 읽어들이기
    let mut f = File::open(filename).expect("file not found");
    */

}

struct Config {
    query: String,
    filename: String,
}

/*
impl Config {
    // Config::new에서 Result반환
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // 인자의 숫자가 몇 개인지 검증 추가
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}*/

impl Config {
    // 인자의 숫자가 몇 개인지 검증 추가
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}