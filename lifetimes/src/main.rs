

/*라이프타임은 댕글링 참조자를 방지*/


// 함수에서의 제네릭 라이프타임


fn main() {

    // 시그니처 내의 모든 참조자들이 동일한 라이프타임 'a를 가지고 있어야 함을 특정한 longest 함수 정의
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } 
        else {
            y
        }
    }

    // 서로 다른 구체적인 라이프타임을 가진 String 값의 참조자들을 이용한 longest 함수의 사용
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // 참조자를 들고 있는 구조체, 따라서 정의 부분에 라이프타임 명시가 필요
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    // 파라미터와 반환 값의 타입이 참조자임에도 불구하고 라이프타임 명시 없이 컴파일되었던, 4장에서 정의한 바 있는 함수
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }

    // 정적 라이프타임 (Static lifetimes)
    let s: &'static str = "I have a static lifetime.";
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}