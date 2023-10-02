fn main() {
    // 1
    let a: &'static str = "你好 🦀";
    println!("{} {}", a, a.len());

    // 4
    let haiku: &'static str = "
    我写下，擦掉，
    再写，再擦，
    然后一朵罂粟花开了。
    - 葛饰北斋";
    println!("{}", haiku);
    println!(
        "你好 \
世界"
    ); // 注意11行 世 字前面的空格会被忽略

    // 5
    let a: &'static str = r#"
    <div class="advice">
        原始字符串在一些情景下非常有用。
    </div>
    "#;
    println!("{}", a);

    // 7
    let a = "你好 🦀";
    println!("{}", a.len());
    let first_word = &a[0..6];
    let second_word = &a[7..11];
    // let half_crab = &a[7..9]; 报错
    // Rust 不接受无效 unicode 字符构成的片段
    println!("{} {}", first_word, second_word);

    // 8
    // 收集字符并转换为类型为 char 的 vector
    let chars = "你好 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len()); // 结果应为 4
                                 // 由于 char 为 4 字节长，我们可以将其转化为 u32
    println!("{}", chars[3] as u32);

    // 9
    let mut helloworld = String::from("你好");
    helloworld.push_str(" 世界");
    helloworld = helloworld + "!";
    println!("{}", helloworld);

    // 10
    // say_it_loud can borrow &'static str as a &str
    say_it_loud("你好");
    // say_it_loud can also borrow String as a &str
    say_it_loud(&String::from("再见"));

    // 11
    let helloworld = ["你好", " ", "世界", "！"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}", abc);

    //12
    let a = 42;
    let f = format!("生活诀窍: {}", a);
    println!("{}", f);

    // 13
    // fn main() -> Result<(), std::num::ParseIntError> {
    //     let a = 42;
    //     let a_string = a.to_string();
    //     let b = a_string.parse::<i32>()?;
    //     println!("{} {}", a, b);
    //     Ok(())
    // }
}

// 10
fn say_it_loud(msg: &str) {
    println!("{}！！！", msg.to_string().to_uppercase());
}
