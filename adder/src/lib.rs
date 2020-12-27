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
        }
    }

    // 아래 표현식은 true를 반환할 예정이므로, test를 통과
    #[cfg(test)]
    mod tests {
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
    }
}