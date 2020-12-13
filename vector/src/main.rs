
fn main() {
    // 빈 벡터 생성
    let v: Vec<i32> = Vec::new();

    let vce = vec![1, 2, 3];

    // 벡터 갱신
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // &와 []를 이용하여 참조자를 얻은 것
    let mut third: &i32 = &v[2];

    // get 함수에 인덱스를 파라미터로 넘겨서 Option<&T>를 얻은 것
    let mut third: Option<&i32> = v.get(2);

    // 각 요소들에 대한 작업
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 모든 요소들에 대한 작업, 가변참조자로 반복
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // 열거형을 정의하여 벡터 내에 다른 타입의 데이터를 담을 수 있도록 함
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
