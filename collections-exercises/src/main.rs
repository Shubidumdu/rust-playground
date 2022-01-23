use std::collections::HashMap;
use std::io;

fn get_median(mut nums: Vec<i32>) -> f32 {
    nums.sort();
    let len = nums.len();
    if len % 2 == 1 {
        let median = nums[len / 2];
        median as f32
    } else {
        let median = (nums[len / 2 - 1] as f32 + nums[len / 2] as f32) / 2.0;
        median
    }
}

fn get_mode(nums: &Vec<i32>) -> i32 {
    let mut num_table = HashMap::new();
    for n in nums {
        let num = num_table.entry(n).or_insert(0);
        *num += 1;
    }
    println!("{:?}", num_table);
    let mode = num_table.iter().max_by(|x, y| x.1.cmp(&y.1)).map(|(k, _v)| k);
    match mode {
        Some(n) => **n,
        None => panic!("There's no value."),
    }
}

fn to_pig_latin(str: String) -> String {
    let first_letter = str.chars().next().unwrap();
    let rest_letter = &str[1..];
    match first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            format!("{}-hay", str)
        }
        _ => {
            format!("{}-{}ay", rest_letter, first_letter)
        }
    }
}

fn employee_sys() {
    let departments : Vec<String> = Vec::new();
    let employees : HashMap<String, String> = HashMap::new();

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
    }
}

fn main() {
    let nums = vec![5, 4, 3, 2, 2, 1];
    let mode = get_mode(&nums);
    println!("The mode of nums is {}", mode);
    let median = get_median(nums);
    println!("The median of nums is {}", median);
    let pig_latin_consonant = to_pig_latin(String::from("first"));
    let pig_latin_vowel = to_pig_latin(String::from("apple"));
    println!("{}", pig_latin_consonant);
    println!("{}", pig_latin_vowel);
}
