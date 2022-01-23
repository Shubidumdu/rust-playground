enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // create
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    // update
    let v3 = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // drop
    {
        let v = vec![1, 2, 3, 4];

        // ...
    }   // v goes out of scope and is freed here.

    // read
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // 해당 index에 값이 없다면 이는 panic을 유발합니다.
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        // 반면 get 메서드의 경우는 panic이 아니라 None을 반환합니다.
        None => println!("There is no third element"),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(24.0),
        SpreadsheetCell::Text(String::from("하이")),
    ];
}
