fn main() {
    // create
    let mut s = String::new();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");

    // update
    {
        let mut s = String::from("foo");
        s.push_str("bar");

        let mut s2 = String::from("foo");
        let s3 = "bar";
        s2.push_str(s3);
        println!("s3 is {}", s3)

        let mut s4 = String::from("lo");
        s4.push('l');
    }

    // concatenation
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
    
        let s = s1 + "-" + &s2 + "-" + &s3;
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        
        let s = format!("{}-{}-{}", s1, s2, s3);

        println!("{}-{}-{}", s1, s2, s3);
    }

    // slicing
    // indexing은 불가능하고, slicing만이 가능합니다.
    {
        
        let hello = "Здравствуйте";
        let s = &hello[0..4];
    }

    // iterating
    {
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }
    
        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}
