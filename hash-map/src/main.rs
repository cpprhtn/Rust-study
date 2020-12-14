
fn main() {
    use std::collections::HashMap;

    // 새로운 해쉬맵을 생성하여 몇 개의 키와 값 부여
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 팀의 리스트와 점수의 리스트로부터 해쉬맵 생성
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 키와 값이 삽입되는 순간 이들이 해쉬맵의 소유가 됨
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value은 이 지점부터 유효하지 않음

    // 해쉬맵 내에 저장된 블루 팀의 점수 접근
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    //
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 값 덮어쓰기
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // entry 메소드를 이용하여 어떤 키가 값을 이미 갖고 있지 않을 경우에만 추가
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 단어의 등장 횟수 카운트
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
