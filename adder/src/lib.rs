
pub fn add_two(a: i32) -> i32 {
    a + 2
    // 아래 코드를 돌릴시에는
    // it_adds_two에서 FAILED가 뜸
    // a + 3
}

// greeting_contains_name 성공 ver
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// greeting_contains_name 실패 ver
pub fn greeting2(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    #[test] // 테스트 함수임을 나타내는 역할
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    /*&
    // 이부분은 실패
    // 실패사유 : panicked at 'Make this test fail'
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
    */

    // assert! 메크로를 이용해 결과 확인
    /*
    assert! 매크로는 테스트가 어떤 조건이 true임을 보장하기를 원하는 경우 유용
    assert! 매크로에는 부울린 타입으로 계산되는 인자가 제공
    만일 값이 true라면 assert!는 아무일도 하지 않고 테스트는 통과
    만일 값이 false라면, assert!는 panic! 매크로를 호출하는데, 이것이 테스트를 실패
    이는 우리의 코드가 우리 의도대로 기능하고 있는지를 체크하는 것을 도와주는 매크로 중 하나
    */
    #[derive(Debug)]
    pub struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
            // 아래 코드(부등호 방향 반대)를 돌릴시에는 
            // larger_can_hold_smaller에서 FAILED가 뜸
            // self.length < other.length && self.width > other.width
        }
    }

    // 아래 표현식은 true를 반환할 예정이므로, test를 통과
    
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    // 이경우에는 can_hold가 false를 반환할 경우에만 통과
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    // assert_eq! 매크로를 이용하는 add_two 함수 테스트
    /*
    assert_eq! 매크로에 제공한 첫번째 인자 4는 add_two(2) 호출의 결과와 동일 
    이 테스트에 대한 라인은 test tests::it_adds_two ... ok이고, ok 문자열은 테스트가 통과했음을 나타냄
    */
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // 커스텀 실패 메세지 추가하기
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // 그저 단언이 실패했으며 몇 번째 줄의 단언이 실패했는지만을 나타냄
    #[test]
    fn greeting_contains_name2() {
        let result = greeting2("Carol");
        assert!(result.contains("Carol"));
    }

    // 실패시 greeting 함수로부터 얻은 값을 출력하도록 만듦
    #[test]
    fn greeting_contains_name3() {
        let result = greeting2("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

}