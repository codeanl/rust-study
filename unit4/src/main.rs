// 1
// 一个部分定义的结构体类型
struct BagOfHolding<T> {
    item: T,
}

// 2
enum Item {
    Inventory(String),
    // None represents the absence of an item
    None,
}
struct BagOfHolding {
    item: Item,
}

// 3
// 一个部分定义的结构体
struct BagOfHolding<T> {
    // 我们的参数类型T可以传递给其他
    item: Option<T>,
}

// 4
fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

// 5
fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() {
    // 1
    // 注意：通过使用泛型，我们创建了编译时创建的类型，使代码更大
    // Turbofish 使之显式化
    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: true };

    // Rust 也可以推断出泛型的类型！
    let float_bag = BagOfHolding { item: 3.14 };

    // 注意：在现实生活中，不要把一袋东西放在另一袋东西里:)
    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "嘭！" },
    };

    println!(
        "{} {} {} {}",
        i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
    );

    // 3
    // 注意：一个放 i32 的 bag，里面什么都没有！
    // 我们必须注明类型，否则 Rust 不知道 bag 的类型
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("there's nothing in the bag!")
    } else {
        println!("there's something in the bag!")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("there's something in the bag!")
    } else {
        println!("there's nothing in the bag!")
    }

    // match 可以让我们优雅地解构 Option，并且确保我们处理了所有的可能情况！
    match i32_bag.item {
        Some(v) => println!("found {} in bag!", v),
        None => println!("found nothing"),
    }

    // 4
    let result = do_something_that_might_fail(12);

    // match 让我优雅地解构 Rust，并且确保我们处理了所有情况！
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}", e),
    }

    // 5
    // fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    //     if i == 42 {
    //         Ok(13.0)
    //     } else {
    //         Err(String::from("this is not the right number"))
    //     }
    // }
    // // 主函数不返回值，但可能返回一个错误！
    // fn main() -> Result<(), String> {
    //     let result = do_something_that_might_fail(12);
    //     match result {
    //         Ok(v) => println!("found {}", v),
    //         Err(_e) => {
    //             // 优雅地处理错误
    //             // 返回一个说明发生了什么的新错误！
    //             return Err(String::from("something went wrong in main!"));
    //         }
    //     }
    //     // Notice we use a unit value inside a Result Ok
    //     // to represent everything is fine
    //     Ok(())
    // }

    // 6
    // fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    //     if i == 42 {
    //         Ok(13.0)
    //     } else {
    //         Err(String::from("this is not the right number"))
    //     }
    // }
    // fn main() -> Result<(), String> {
    //     // 看看我们节省了多少代码！
    //     let v = do_something_that_might_fail(42)?;
    //     println!("found {}", v);
    //     Ok(())
    // }

    // 7
    // fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    //     if i == 42 {
    //         Ok(13.0)
    //     } else {
    //         Err(String::from("this is not the right number"))
    //     }
    // }
    // fn main() -> Result<(), String> {
    //     // 简洁但假设性强，而且很快就会变得丑陋
    //     let v = do_something_that_might_fail(42).unwrap();
    //     println!("found {}", v);

    //     // 这会 panic!
    //     let v = do_something_that_might_fail(1).unwrap();
    //     println!("found {}", v);
    //     Ok(())
    // }

    // 8
    // 我们可以显式确定类型
    let mut i32_vec = Vec::<i32>::new(); // turbofish <3
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    // 但是看看 Rust 是多么聪明的自动检测类型啊
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // 这是个漂亮的宏！
    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in string_vec.iter() {
        println!("{}", word);
    }
}
