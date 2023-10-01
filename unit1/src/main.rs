fn main() {
    // 1.
    // rust 推断出x的类型
    let x = 13;
    println!("{}", x);
    // rust也可以显式声明类型
    let x: f64 = 3.14159;
    println!("{}", x);
    // rust 也支持先声明后初始化，但很少这样做
    let x;
    x = 0;
    println!("{}", x);

    // 2.
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);

    // 3.
    let x = 12; // 默认情况下，这是i32
    let a = 12u8;
    let b = 4.3; // 默认情况下，这是f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );

    // 4.
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);
    let t = true;
    println!("{}", t as u8);

    // 4.
    const PI: f32 = 3.14159;
    println!(
        "To make an apple {} from scratch, you must first create a universe.",
        PI
    );

    // 5.
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);

    // 6.
    println!("{}", add(42, 13));

    // 7.
    // 返回一个元组
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);
    // 将元组解构为两个变量
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);

    // 8
    let a = make_nothing();
    let b = make_nothing2();
    // 打印a和b的debug字符串，因为很难去打印空
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}

// 6
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
// 7
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

// 8
fn make_nothing() -> () {
    return ();
}
// 返回类型隐含为 ()
fn make_nothing2() {
    // 如果没有指定返回值，这个函数将会返回 ()
}
