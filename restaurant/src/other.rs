// `{}` 블록이 아니라 모듈 이름 뒤에 곧바로 `;`이 붙는 경우 동일한 이름으로 작성된 다른 파일의 모듈을 사용한다는 의미입니다.
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
}