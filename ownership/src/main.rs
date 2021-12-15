fn main() {
    println!("Hello, world!");
}

fn make_str_mut() {
    let mut s = String::from("hello"); // 이제 mutable 합니다.
    s.push_str(", world!");
    println!("{}", s);
}

fn ownership() {
    let s = String::from("hello");
    takes_ownership(s); // s의 값이 함수로 넘어가고, 더이상 s는 유효하지 않습니다.

    let x = 5;
    makes_copy(x); // x의 값이 함수로 넘어가지만, i32는 Copy이기 때문에, 여전히 x는 유효합니다.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}