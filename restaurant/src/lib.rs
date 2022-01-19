mod front_of_house {
    // 각각의 모듈 및 내용을 외부에서 접근 및 사용하려면 각각에 pub 선언을 해주어야 합니다.
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        
        fn serve_order() {}

        mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                // super는 터미널 상에서의 `..`와 같은 역할을 합니다.
                super::serve_order();
            }

            fn cook_order() {}
        }

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}


pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 아래 주석은 컴파일 에러를 유발할 겁니다. (private함)
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant3() {
    // 반면, `enum`의 경우에는 `enum` 자체에 pub을 적용하면 하위 내용 모두 public이 됩니다.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// use 키워드를 사용하면 해당 모듈을 현재 스코프로 가져올 수 있습니다.
use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
// use front_of_house::hosting;

use hosting as pubHosting;

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 이처럼 완전히 해당 모듈의 함수를 스코프로 가져올 수도 있지만, 이 경우 로컬 함수와의 혼동의 여지가 있습니다.
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant5() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// 한편, struct나 enum을 가져다 쓰는 경우에는 이 쪽이 더 관용적입니다. 이는 일종의 컨벤션에 가깝습니다.
use std::collections::HashMap;

fn test_hash() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// 물론 예외는 있는데, 동일한 이름을 가진 struct나 enum을 가져와야할 경우에는 parent 레벨을 가져올 수 있습니다.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

// 또 다른 해결 방법도 있는데, `as` 키워드로 새로운 alias로 만들어버리는 것입니다.
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
}

fn function4() -> IoResult<()> {
    // --snip--
}

// pub use는 re-exporting 이라고 합니다.
// 기본적으로 use는 해당 스코프에 private하게 사용되는데, pub use의 경우 외부에서도 이를 사용할 수 있게 됩니다.
pub use crate::front_of_house::hosting;

// 중복된 경로에 위치한 여러 아이템들을 한번에 가져오려면 아래와 같이 사용할 수 있습니다.
use std::{cmp::Ordering, io};
use std::io::{self, Write}; // 이는 std::io와 std::io::Write를 가져옵니다.

// 특정 경로의 모든 public 아이템들을 가져오려면 `*`를 사용하세요. 하지만 스코프 관리가 어렵기 때문에 이는 권장되진 않습니다.
// 이는 테스트 모듈의 작성을 위해 종종 사용됩니다.
use std::collections::*;