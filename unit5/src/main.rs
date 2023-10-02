fn main() {
    // 1
    // 我们实例化这个结构体并将其绑定到具体的变量上
    // 来创建内存资源
    let foo = Foo { x: 42 };
    // foo 即为该资源的所有者

    // 2
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };
    println!("{}", foo_a.x);
    // foo_a 将在这里被 dropped 因为其在这之后再也没有被使用
    println!("{}", foo_b.x);
    // foo_b 将在这里被 dropped 因为这是函数域的结尾

    // 3
    let foo = Foo { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);
    // foo 首先被 dropped 释放
    // 紧接着是 foo.bar

    // 4
    let foo = Foo { x: 42 };
    // foo 被移交至 do_something
    do_something(foo);
    // 此后 foo 便无法再被使用

    // 5
    let foo = do_something();
    // foo 成为了所有者
    // foo 在函数域结尾被 dropped 释放

    // 6
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x);
    // f 在这里被 dropped 释放
    // foo 在这里被 dropped 释放

    // 7
    let mut foo = Foo { x: 42 };
    let f = &mut foo;
    // 会报错: do_something(foo);
    // 因为 foo 已经被可变借用而无法取得其所有权
    // 会报错: foo.x = 13;
    // 因为 foo 已经被可变借用而无法被修改
    f.x = 13;
    // f 会因为此后不再被使用而被 dropped 释放
    println!("{}", foo.x);
    // 现在修改可以正常进行因为其所有可变引用已经被 dropped 释放
    foo.x = 7;
    // 移动 foo 的所有权到一个函数中
    do_something(foo);

    // 8
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // 取得所有者值的拷贝
    *f = 13; // 设置引用所有者的值
    println!("{}", bar);
    println!("{}", foo);

    // 9
    let mut foo = Foo { x: 42 };
    do_something(&mut foo);
    // 因为所有的可变引用都在 do_something 函数内部被释放了
    // 此时我们便可以再创建一个
    do_something(&mut foo);
    // foo 在这里被 dropped 释放

    // 10
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x 在这里被 dropped 释放从而允许我们再创建一个不可变引用
    let y = do_something(&foo);
    println!("{}", y);
    // y 在这里被 dropped 释放
    // foo 在这里被 dropped 释放

    // 11
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x 在这里被 dropped 释放从而允许我们再创建一个不可变引用
    let y = do_something(&foo);
    println!("{}", y);
    // y 在这里被 dropped 释放
    // foo 在这里被 dropped 释放

    // 12
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };
    let x = do_something(&foo_a, &foo_b);
    // foo_a 在这里被 dropped 释放因为只有 foo_b 的生命周期在此之后还在延续
    println!("{}", x);
    // x 在这里被 dropped 释放
    // foo_b 在这里被 dropped 释放

    // 13
    // 静态变量的范围也可以被限制在一个函数内
    static mut SECRET: &'static str = "swordfish";

    // 字符串字面值拥有 'static 生命周期
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // 你可以打破一些规则，但是必须是显式地
    unsafe {
        // 我们可以修改 SECRET 到一个字符串字面值因为其同样是 'static 的
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }

    // 14
    let x = 42;
    let foo = Foo { i: &x };
    println!("{}", foo.i);
}

// 1 2
// struct Foo {
//     x: i32,
// }

// 3
struct Bar {
    x: i32,
}
struct Foo {
    bar: Bar,
}

// 4
struct Foo {
    x: i32,
}
fn do_something(f: Foo) {
    println!("{}", f.x);
    // f 在这里被 dropped 释放
}

// 5
struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // 所有权被移出
}

// 6
struct Foo {
    x: i32,
}

// 7
struct Foo {
    x: i32,
}
fn do_something(f: Foo) {
    println!("{}", f.x);
    // f 在这里被 dropped 释放
}

// 9
struct Foo {
    x: i32,
}
fn do_something(f: &mut Foo) {
    f.x += 1;
    // 可变引用 f 在这里被 dropped 释放
}

// 10
struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x;
}

// 11
struct Foo {
    x: i32,
}

// 参数 foo 和返回值共享同一生命周期
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}

// 12
struct Foo {
    x: i32,
}

// foo_b 和返回值共享同一生命周期
// foo_a 则拥有另一个不相关联的生命周期
fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    return &foo_b.x;
}

// 13
static PI: f64 = 3.1415;

// 14
struct Foo<'a> {
    i: &'a i32,
}
