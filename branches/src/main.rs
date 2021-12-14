fn main() {
  // Convert between Fahrenheight and Celsius
  let temp = convert_temp(50., 'c');
  println!("The temperature is {}", temp);

  // Fibonaci number
  let target = 20;
  let fibo = fibo_num(target);
  println!("Fibonaci {} is {}", target, fibo);

  // Sing The Twelve Days of Christmas
  sing_carole();
}

fn sing_carole() {
  let lyrics = [
    "A song and a Christmas tree.",
    "Two candy canes",
    "Three boughs of holly",
    "Four colored lights,",
    "A shining star.",
    "Little silver bells",
    "Candles a-glowing,",
    "Gold and silver tinsel,",
    "A guardian angel,",
    "Some mistletoe,",
    "Gifts for one and all,",
    "All their good wishes,"
  ];
  let days = [
    "First",
    "Second",
    "Third",
    "Fourth",
    "Fifth",
    "Sixth",
    "Seventh",
    "Eighth",
    "Ninth",
    "Tenth",
    "Eleventh",
    "Twelfth"
  ];
  for index in 0..12 {
    println!("On the {} day of Christmas", days[index]);
    println!("My good friends brought to me");
    let mut lyrics_index = 0;
    while lyrics_index <= index {
      println!("{}", lyrics[index - lyrics_index]);
      lyrics_index += 1;
    }
    println!();
  }
}

fn fibo_num(n: u16) -> u32 {
  let mut num_1 = 1;
  let mut num_2 = 1;
  if n == 1 || n == 2 {
    return 1;
  } else {
    let mut index = 2;
    let result = loop {
      index += 1;
      let result = num_1 + num_2;
      if index == n {
        break result;
      }
      if index % 2 == 1 {
        num_1 = result;
      } else {
        num_2 = result;
      }
    };
    return result;
  }
}

fn convert_temp(temp: f64, target: char) -> f64 {
  if target == 'f' {
    return temp * 1.8 + 32.;
  } else if target == 'c' {
    return (temp - 32.) * 0.5556;
  } else {
    return temp;
  }
}

fn condition() {
  let number = 7;
  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }
}

// fn number_three() {
//   let number = 3;
//   // Rust는 JS나 Ruby같은 언어와 달리 자동적인 타입 변환이 일어나지 않습니다.
//   // Rust는 condition을 따질때 반드시 `bool` 타입의 값이어야 합니다.
//   if number {
//     println!("number was three");
//   }
// }

fn else_if() {
  let number = 6;
  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }
}

fn let_if() {
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {}", number);
}

fn make_loop() {
  loop {
    println!("again!");
  }
}

fn named_loop() {
  let mut count = 0;
  // loop에는 별도로 이름을 붙일 수 있습니다.
  'counting_up: loop {
    println!("count = {}", count);
    let mut remaining = 10;
    loop {
        println!("remaining = {}", remaining);
        if remaining == 9 {
            break;
        }
        if count == 2 {
            // 종료하고자 하는 루프를 직접 지정할 수 있습니다.
            break 'counting_up;
        }
        remaining -= 1;
    }
    count += 1;
  }
}

fn loop_return() {
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {}", result);
}

fn loop_while() {
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }
  println!("LIFTOFF!!!");
}

fn for_iter() {
  let a = [10, 20, 30, 40, 50];
  for element in a {
    println!("the value is: {}", element);
  }
}

fn for_iter_reverse() {
  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}

