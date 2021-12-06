fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    use_constant();
    use_shadowing();
}

fn use_constant() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn use_shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // 아래는 정상적으로 동작합니다.
    // 두번째로 쓰인 spaces는 아예 새로운 shadowing 변수입니다.
    let spaces = "   ";
    let spaces = spaces.len();

    // 반면 아래는 에러가 발생합니다.
    // spaces의 타입과 spaces.len() 반환값이 타입이 일치하지 않기 때문입니다.
    let mut spaces = "   ";
    spaces = spaces.len();
}
