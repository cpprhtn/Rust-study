// * 배포되어있는 내용은 현재 버전에서 돌아가지 않음
// 공식 문서를 참고할 것


/*
fn main() {
    
    // summary 메소드에 의해 제공되는 동작으로 구성된 Summarizable 트레잇의 정의
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // NewsArticle과 Tweet 타입 상에서의 Summarizable 트레잇 구현
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }


    // 출력 결과 : 1 new tweet: horse_ebooks: of course, as you probably already know, people
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    // summary 메소드의 기본 구현을 포함한 Summarizable 트레잇의 정의
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle {}
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    

    // 출력 결과 : New article available! (Read more...)
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    


    // 아래 버전의 Summarizable을 사용하기 위해서는, 어떤 타입에 대한 이 트레잇을 구현할 때 author_summary만 정의하기
    pub trait Summary {
        fn summarize_author(&self) -> String;
    
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }


    // 출력 결과 : 1 new tweet: (Read more from @horse_ebooks...)
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
*/


/*
// - largest특성 경계로 함수 수정
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
*/


// - 특성 경계를 사용하여 메서드를 조건부로 구현
fn main() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}