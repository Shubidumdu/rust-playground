pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
      format!("(Read more from {}...)", self.summarize_author())
  }
}

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

  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
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

  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}


// 아래 `notify`는 trait bound를 이용해 들어오는 item의 trait를 제한합니다.
// 사실 아래의 notify와 동일합니다.
// pub fn notify<T: Summary>(item: &T) {
//   println!("Breaking news! {}", item.summarize());
// }
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// 만약 여러 개의 impl 파라미터를 가져오는 상황이라면
// 1. 각각의 타입을 다르게 지정하고 싶다면

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// 2. 두 타입을 동일하게 강제하고 싶다면
// pub fn notify<T: Summary>(item1: &T, item2: &T) {


// 여러 trait를 충족하는 매개변수를 지정하고 싶다면
// pub fn notify(item: &(impl Summary + Display)) {
// 이건 generic에도 사용할 수 있습니다.
// pub fn notify<T: Summary + Display>(item: &T) {

// trait bounds를 너무 많이 사용하면 가독성이 떨어집니다.
// 그렇기 때문에 `where` 문을 사용할 수 있습니다.
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {

// 반환 타입에도 trait를 이용할 수 있습니다.
pub fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  }
}