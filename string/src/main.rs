
fn main() {
    let mut s = String::new();

    //
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    //
    let s = String::from("initial contents");

    // 스트링 갱신
    let mut s = String::from("foo");
    s.push_str("bar");

    //
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);

    //
    let mut s = String::from("lo");
    s.push('l');

    //
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 여기서 이동되어 더이상 쓸 수 없음을 유의하세요

    //
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    //
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    //
    let len = String::from("Hola").len();
    
    // 스트링 슬라이싱
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    //
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
