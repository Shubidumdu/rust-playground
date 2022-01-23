use std::collections::HashMap;

fn main() {
    {
        // creating
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        // `collect`는 여러 데이터 구조로 변환할 수 있기 떄문에, 결과가 `HashMap`임을 명시해주어야 합니다.
        let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
    
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // Copy trait를 구현하는 타입의 경우에는 값이 복사되지만,
        // String과 같은 Owned value인 경우에는 값이 이동합니다.
        // 따라서 더 이상 field_name과 field_value에 접근할 수 없습니다.
    }

    {
        // accessing
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    
        let team_name = String::from("Blue");
        let score = scores.get(&team_name); // Some(&10) -> get은 Option<&V>을 반환합니다.

        // iterating
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    {
        // updating
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25); // 값이 25로 덮어씌워집니다.
        
        println!("{:?}", scores);
    }

    {
        // entry
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
    
        // entry는 값이 있는지 없는지를 나타내는 mut Entry enum을 반환하는 메서드입니다.
        // `or_insert` 메서드를 통해 비어있는 경우에는 값을 넣을 수 있습니다.
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50); // blue팀에는 이미 값이 있으므로 삽입되지 않습니다.
    
        println!("{:?}", scores);
    }

    {
        // updating based on old value
        let text = "hello world wonderful world";

        let mut map = HashMap::new();
    
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
    
        println!("{:?}", map);
    }
}
