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

    2.
}
