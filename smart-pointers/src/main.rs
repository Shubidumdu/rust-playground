use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 커스텀 타입에 deref `*`를 사용하려면
// `Deref` 트레잇에 대한 impl을 작성해주어야 합니다.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct CustomSmartPointer {
    data: String,
}

// `Drop` 트레잇은 스코프가 끝난 경우,
// 인스턴스를 어떻게 처리할지에 대한 cleanup의 역할을 합니다.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    {    // Box는 데이터를 heap에 할당할 수 있도록 해줍니다.
        let b = Box::new(5);
        println!("b = {}", b);

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    {
        // x에 대한 참조 y가 가리키는 값에 접근하려면 
        // `*` deref 연산자가 필요합니다.
        let x = 5;
        let y = &x;
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        // `Box`는 참조와 동일한 역할을 할 수 있습니다.
        // 다만 참조 포인터가 아닌 Box 인스턴스를 생성한다는 차이가 있습니다.
        let x = 5;
        let y = Box::new(x);
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        // Deref 트레잇이 작성된 경우,
        // `*y`는 실제로는 `*(y.deref())`와 같이 동작합니다.
        assert_eq!(5, *y);
    }

    {
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        fn main() {
            let m = MyBox::new(String::from("Rust"));
            hello(&m);
        }
    }

    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        // `drop`을 이용하면 임의로 값을 버릴 수 있습니다.
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }

    {
        // Rc는 하나의 데이터를 여러 군데에서 소유하도록 할 수 있습니다.
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}
