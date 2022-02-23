#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    // 매개변수가 두개라면 둘 사이를 비교합니다. (==)
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn larger_can_hold_smaller() {
      let larger = Rectangle {
          width: 8,
          height: 7,
      };
      let smaller = Rectangle {
          width: 5,
          height: 1,
      };

      // 단일 param의 경우 결과가 true라면 통과합니다.
      assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn it_adds_two() {
      assert_eq!(4, add_two(2));
      assert_ne!(3, add_two(2)); // 한편, assert_ne!는 동일하지 않은 경우에 통과합니다. (!=)
  }

  #[test]
  fn greeting_contains_name() {
      let result = greeting("Carol");
      assert!(
          result.contains("Carol"),
          "Greeting did not contain name, value was `{}`", 
          // 한편, 추가적인 매개변수를 넘기게 되면, 해당 메시지가 Failure 시에 출력됩니다.
          result
      );
  }

  #[test]
  #[should_panic] // should_panic 속성이 추가되면 해당 테스트는 panic
  fn greater_than_100() {
      Guess::new(200);
  }


  // 다만, should_panic은 어떠한 원인으로든 panic!이 발생하면 통과가 되므로,
  // 좀 더 정확한 테스트를 위해서는 옵셔널한 `expected` 파라미터가 필요합니다.
  #[test]
  #[should_panic(expected = "Guess value must be less than or equal to 100")]
  fn greater_than_100() {
      Guess::new(200);
  }

  #[test]
  fn it_works() -> Result<(), String> {
      if 2 + 2 == 4 {
          Ok(())
      } else {
          Err(String::from("two plus two does not equal four"))
      }
  }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
  a + 2
}

pub fn greeting(name: &str) -> String {
  format!("Hello {}!", name)
}

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
      if value < 1 {
          panic!(
              "Guess value must be greater than or equal to 1, got {}.",
              value
          );
      } else if value > 100 {
          panic!(
              "Guess value must be less than or equal to 100, got {}.",
              value
          );
      }

      Guess { value }
  }
}

