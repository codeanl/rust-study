# rust-study

## 1.基础知识
### 1.1 变量
变量使用 let 关键字来声明。
在赋值时，Rust 能够在 99% 的情况下自动推断其类型。如果不能，你也可以手动将类型添加到变量声明中。
你也许注意到了，我们可以对同一个变量名进行多次赋值。这就是所谓的变量隐藏，可以更改变量类型以实现对该变量名的后续使用。
变量名总是遵循 蛇形命名法 (snake_case)。
```
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
```
### 1.2 修改变量
Rust 非常关心哪些变量是可修改的。值分为两种类型：

- 可变的 - 编译器允许对变量进行读取和写入。
- 不可变的 - 编译器只允许对变量进行读取。
可变值用 mut 关键字表示。

关于这个概念，我们之后还会有更多的内容，但是眼下请谨记这个关键字即可。
```
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
```
### 1.3 基本类型
Rust 有多种常见的类型：

布尔型 - bool 表示 true 或 false
无符号整型- u8 u32 u64 u128 表示正整数
有符号整型 - i8 i32 i64 i128 表示正负整数
指针大小的整数 - usize isize 表示内存中内容的索引和大小
浮点数 - f32 f64
元组（tuple） - (value, value, ...) 用于在栈上传递固定序列的值
数组 - 在编译时已知的具有固定长度的相同元素的集合
切片（slice） - 在运行时已知长度的相同元素的集合
str(string slice) - 在运行时已知长度的文本
文本可能比你在其他语言中学到的更复杂，因为 Rust 是一种系统编程语言，它关心的是你可能不太习惯的内存问题。 我们之后将详细讨论这个问题。

另外，你也可以通过将类型附加到数字的末尾来明确指定数字类型（如 13u32 和 2u8）
```
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
```
### 1.4 基本类型转换
当涉及到数字类型时，Rust 要求明确。一个人不能想当然地把“u8”用在“u32”上而不出错。
幸运的是，使用 as 关键字，Rust 使数字类型转换非常容易。
```
let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);
    let t = true;
    println!("{}", t as u8);
```

### 1.5 常量
常量允许我们高效地指定一个在代码中会被多次使用的公共值。不同于像变量一样在使用的时候会被复制，常量会在编译期间直接用它们的值来替换变量的文本标识符。
不同于变量，常量必须始终具有显式的类型。
常量名总是遵循 全大写蛇形命名法（SCREAMING_SNAKE_CASE）。
```
    const PI: f32 = 3.14159;
    println!(
        "To make an apple {} from scratch, you must first create a universe.",
        PI
    );
```
### 1.6 数组
数组是所有相同类型数据元素的固定长度集合。
一个数组的数据类型是 [T;N]，其中 T 是元素的类型，N 是编译时已知的固定长度。
可以使用 [x] 运算符提取单个元素，其中 x 是所需元素的 usize 索引（从 0 开始）。
```
let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
```

### 1.7 函数
函数可以有 0 个或者多个参数。
在这个例子中，add 接受类型为 i32（32 位长度的整数）的两个参数。
函数名总是遵循 蛇形命名法 (snake_case)。
```
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    println!("{}", add(42, 13));
}
```

### 1.8 多个返回值
函数可以通过元组来返回多个值。
元组元素可以通过他们的索引来获取。
Rust 允许我们将后续会看到的各种形式的解构，也允许我们以符合逻辑的方式提取数据结构的子片段。敬请期待后面的内容！
```
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}
fn main() {
    // 返回一个元组
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // 将元组解构为两个变量
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
```

### 1.9 返回空值
如果没有为函数指定返回类型，它将返回一个空的元组，也称为单元。
一个空的元组用 () 表示。
直接使用 () 的情况相当不常见。但它经常会出现（比如作为函数返回值），所以了解其来龙去脉非常重要。
```
fn make_nothing() -> () {
    return ();
}
// 返回类型隐含为 ()
fn make_nothing2() {
    // 如果没有指定返回值，这个函数将会返回 ()
}
fn main() {
    let a = make_nothing();
    let b = make_nothing2();
    // 打印a和b的debug字符串，因为很难去打印空
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}

```

## 2 基本控制流
### 2.1 if/else if/else
Rust 中的代码分支不足为奇。
Rust 的条件判断没有括号！~~需要括号干什么。~~我们现有的逻辑就看起来就很干净整洁呀。
不过呢，所有常见的逻辑运算符仍然适用：==，!=， <， >， <=， >=， !， ||， &&

```
 let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
```

### 2.2 循环
需要一个无限循环？
使用 Rust 很容易实现。
break 会退出当前循环。但 loop 还有个秘密，我们很快讲到。
```
 let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }
    println!("{}", x);
```

### 2.3 while
while 允许你轻松地向循环添加条件。

如果条件一旦变为 false，循环就会退出。
```
 let mut x = 0;
    while x != 42 {
        x += 1;
    }
```

### 2.4 for
Rust 的 for 循环是一个强大的升级。它遍历来自计算结果为迭代器的任意表达式的值。 迭代器是什么？迭代器是一个你可以一直询问“下一项是什么？”直到没有其他项的对象。
我们将在以后的章节中进一步探讨这一点，与此同时，我们知道 Rust 使创建生成整数序列的迭代器变得容易。
. .. 运算符创建一个可以生成包含起始数字、但不包含末尾数字的数字序列的迭代器。
. ..= 运算符创建一个可以生成包含起始数字、且包含末尾数字的数字序列的迭代器。
```
  for x in 0..5 {
        println!("{}", x);
    }

    for x in 0..=5 {
        println!("{}", x);
    }
```

### 2.5 match
想念你的 switch 语句吗？Rust 有一个非常有用的关键字，用于匹配值的所有可能条件， 并在匹配为真时执行相应代码。我们先来看看对数字的使用。在未来章节中，我们将有更多 更复杂的数据模式匹配的说明，我向你保证，它将值得等待。
match 是穷尽的，意为所有可能的值都必须被考虑到。
匹配与解构相结合是迄今为止你在 Rust 中看到的最常见的模式之一。
```
   let x = 42;
    match x {
        0 => {
            println!("found zero");
        }
        // 我们可以匹配多个值
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // 我们可以匹配迭代器
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // 我们可以将匹配数值绑定到变量
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // 这是默认匹配，如果没有处理所有情况，则必须存在该匹配
        _ => {
            println!("found something else!");
        }
    }
```

### 2.6 从循环中返回值
loop 可以被中断以返回一个值。
```
 let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);
```

### 2.7 从块表达式返回值
if，match，函数，以及作用域块都有一种返回值的独特方式。
如果 if、match、函数或作用域块中的最后一条语句是不带 ; 的表达式， Rust 将把它作为一个值从块中返回。这是一种创建简洁逻辑的好方法，它返回一个 可以放入新变量的值。
注意，它还允许 if 语句像简洁的三元表达式一样操作。
```
fn example() -> i32 {
    let x = 42;
    // Rust的三元表达式
    let v = if x < 42 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hamburger";
    let result = match food {
        "hotdog" => "is hotdog",
        // 注意，当它只是一个返回表达式时，大括号是可选的
        _ => "is not hotdog",
    };
    println!("identifying food: {}", result);
    let v = {
        // 这个作用域块让我们得到一个不影响函数作用域的结果
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);
    // 在最后从函数中返回值的惯用方法
    v + 4
}

fn main() {
    println!("from function: {}", example());
}
```

##  基本数据结构类型
### 3.1 结构体
一个 struct 就是一些字段的集合。
字段是一个与数据结构相关联的数据值。它的值可以是基本类型或结构体类型。
它的定义就像给编译器的蓝图，告诉编译器如何在内存中布局彼此相邻的字段。
```
struct SeaCreature {
    // String 是个结构体
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}
```

### 3.2 方法调用
与函数（function）不同，方法（method）是与特定数据类型关联的函数。
. 静态方法 — 属于某个类型，调用时使用 :: 运算符。
. 实例方法 — 属于某个类型的实例，调用时使用 . 运算符。
我们将在后续章节中更多地讨论如何创建自己的方法。
```
  // 使用静态方法来创建一个String实例
    let s = String::from("Hello world!");
    // 使用实例来调用方法
    println!("{} is {} characters long.", s, s.len());
```

### 3.3内存
Rust 程序有 3 个存放数据的内存区域：
. 数据内存 - 对于固定大小和静态（即在整个程序生命周期中都存在）的数据。 考虑一下程序中的文本（例如 “Hello World”），该文本的字节只能读取，因此它们位于该区域中。 编译器对这类数据做了很多优化，由于位置已知且固定，因此通常认为编译器使用起来非常快。
. 栈内存 - 对于在函数中声明为变量的数据。 在函数调用期间，内存的位置不会改变，因为编译器可以优化代码，所以栈数据使用起来比较快。
. 堆内存 - 对于在程序运行时创建的数据。 此区域中的数据可以添加、移动、删除、调整大小等。由于它的动态特性，通常认为它使用起来比较慢， 但是它允许更多创造性的内存使用。当数据添加到该区域时，我们称其为分配。 从本区域中删除 数据后，我们将其称为释放。

### 3.4 在内存中创建数据
当我们在代码中实例化一个结构体时，我们的程序会在内存中并排创建关联的字段数据。
当我们通过制定所有字段值的方式来实例化时：
结构体名 { ... }.
结构体字段可以通过 . 运算符来获取。
我们例子的内存详情：

. 引号内的文本是只读数据（例如“ferris”），因此它位于数据内存区。
. 函数调用 String::from 创建一个结构体 String，该结构体与 SeaCreature 的字段并排放置在栈中。 字符串容器通过如下步骤表示可更改的文本：
在堆上创建可修改文本的内存。
将堆中存储对象的内存位置的引用存储在 String 结构体中（在以后的课程中会详细介绍）。
. 最后，我们的两个朋友 Ferris 和 Sarah 有在程序中总是固定的位置的数据结构，所以它们被放在栈上。
```
  // SeaCreature的数据在栈上
    let ferris = SeaCreature {
        // String 结构体也在栈上，
        // 但也存放了一个数据在堆上的引用
        animal_type: String::from("螃蟹"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("大钳子"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("章鱼"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("无"),
    };
    
    println!(
        "{} 是只{}。它有 {} 只胳膊 {} 条腿，还有一个{}。",
        ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    );
    println!(
        "{} 是只{}。它有 {} 只胳膊 {} 条腿。它没有杀伤性武器…",
        sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    );
```

### 3.5 类元组结构体
简洁起见，你可以创建像元组一样被使用的结构体。
```
struct Location(i32, i32);
fn main() {
    // 这仍然是一个在栈上的结构体
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}

```

### 3.6 类单元结构体
结构体也可以没有任何字段。
就像第一章提到的，一个 unit 是空元组 () 的别称。这就是为什么，此类结构体被称为 类单元。
这种类型的结构体很少用到。
```
struct Marker;

fn main() {
    let _m = Marker;
}

```

### 3.7 枚举
枚举允许你使用 enum 关键字创建一个新类型，该类型的值可以包含几个带标记的元素。
match 有助于确保对所有可能的枚举值进行彻底的处理，使其成为确保高质量代码的强大工具。
```
#![allow(dead_code)] // this line prevents compiler warnings
enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}
struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}
fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };
    match ferris.species {
        Species::Crab => println!("{} is a crab",ferris.name),
        Species::Octopus => println!("{} is a octopus",ferris.name),
        Species::Fish => println!("{} is a fish",ferris.name),
        Species::Clam => println!("{} is a clam",ferris.name),
    }
}

```

### 3.8带数据的枚举
enum 的元素可以有一个或多个数据类型，从而使其表现得像 C 语言中的联合。
当使用 match 对一个 enum 进行模式匹配时，可以将变量名称绑定到每个数据值。
enum 的内存细节：
. 枚举数据的内存大小等于它最大元素的大小。此举是为了让所有可能的值都能存入相同的内存空间。
. 除了元素数据类型（如果有）之外，每个元素还有一个数字值，用于表示它是哪个标签。
其他细节：
. Rust 的 enum 也被称为标签联合 （tagged-union）
. 把类型组合成一种新的类型，这就是人们所说的 Rust 具有 代数类型 的含义
```
#![allow(dead_code)] // this line prevents compiler warnings

enum Species { Crab, Octopus, Fish, Clam }
enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };
    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws,size) => {
                    let size_description = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("ferris is a crab with {} {} claws", num_claws, size_description)
                },
                _ => println!("ferris is a crab with some other weapon")
            }
        },
        _ => println!("ferris is some other animal"),
    }
}
```
## 4 泛型
### 4.1 泛型是什么？
泛型允许我们不完全定义一个 struct 或 enum，使编译器能够根据我们的代码使用情况，在编译时创建一个完全定义的版本。
Rust 通常可以通过查看我们的实例化来推断出最终的类型，但是如果需要帮助，你可以使用 ::<T> 操作符来显式地进行操作， 该操作符也被称为 turbofish （它是我的好朋友！）。
```
// 一个部分定义的结构体类型
struct BagOfHolding<T> {
    item: T,
}

fn main() {
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
}

```

### 4.2 表示空
在其他语言中，关键字 null 用于表示没有值。它给编程语言带来了困难，因为它使我们的程序在与变量字段交互时可能失败。
Rust 没有 null，但这并不代表我们不知道表示空的重要性！我们可以使用一个我们已经了解过的工具来简单地表示这一点。
因为缺少 null 值，这种为一个或多个替代值提供 None 替代表示的模式非常常见， 泛型有助于解决这一难题。
```
enum Item {
    Inventory(String),
    // None represents the absence of an item
    None,
}

struct BagOfHolding {
    item: Item,
}

```

### 4.3 Option
Rust 有一个内置的泛型枚举叫做 Option，它可以让我们不使用 null 就可以表示可以为空的值。
enum Option<T> {
    None,
    Some(T),
}
这个枚举很常见，使用关键字 Some 和 None 可以在任何地方创建其实例。
```
// 一个部分定义的结构体
struct BagOfHolding<T> {
    // 我们的参数类型T可以传递给其他
    item: Option<T>,
}
fn main() {
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
}
```

### 4.4 Result
Rust 有一个内置的泛型枚举叫做 Result，它可以让我们返回一个可能包含错误的值。 这是编程语言进行错误处理的惯用方法。

enum Result<T, E> {
    Ok(T),
    Err(E),
}
注意我们的泛型有多个用逗号分隔的参数化的类型。
这个枚举很常见，使用关键字 Ok 和 Err 可以在任何地方创建其实例。
```
fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))   
    }
}

fn main() {
    let result = do_something_that_might_fail(12);

    // match 让我优雅地解构 Rust，并且确保我们处理了所有情况！
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}",e),
    }
}

```

### 4.5 可失败的主函数
main 函数有可以返回 Result 的能力！
```
fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

// 主函数不返回值，但可能返回一个错误！
fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(12);

    match result {
        Ok(v) => println!("found {}", v),
        Err(_e) => {
            // 优雅地处理错误
            
            // 返回一个说明发生了什么的新错误！
            return Err(String::from("something went wrong in main!"));
        }
    }

    // Notice we use a unit value inside a Result Ok
    // to represent everything is fine
    Ok(())
}

```

### 4.6 优雅地错误处理
Result 如此常见以至于 Rust 有个强大的操作符 ? 来与之配合。 以下两个表达式是等价的：
```
do_something_that_might_fail()?
```
```
match do_something_that_might_fail() {
    Ok(v) => v,
    Err(e) => return Err(e),
}
```
```
fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    // 看看我们节省了多少代码！
    let v = do_something_that_might_fail(42)?;
    println!("found {}", v);
    Ok(())
}
```

### 4.7 丑陋的 Option/Result 处理
当你只是试图快速地写一些代码时，Option/Result 对付起来可能比较无聊。 Option 和 Result 都有一个名为 unwrap 的函数：这个函数可以简单粗暴地获取其中的值。 unwrap 会：
. 获取 Option/Result 内部的值
. 如果枚举的类型是 None/Err， 则会 panic!
这两段代码是等价的：

```my_option.unwrap()
```
```
match my_option {
    Some(v) => v,
    None => panic!("some error message generated by Rust!"),
}
```
类似的：
```
my_result.unwrap()
```
```
match my_result {
    Ok(v) => v,
    Err(e) => panic!("some error message generated by Rust!"),
}
```
不过啊，做个好 Rustacean，正确地使用 match！
```
fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    // 简洁但假设性强，而且很快就会变得丑陋
    let v = do_something_that_might_fail(42).unwrap();
    println!("found {}", v);
    
    // 这会 panic!
    let v = do_something_that_might_fail(1).unwrap();
    println!("found {}", v);
    
    Ok(())
}

```

### 4.8 Vectors
一些经常使用的泛型是集合类型。一个 vector 是可变长度的元素集合，以 Vec 结构表示。
比起手动构建，宏 vec! 让我们可以轻松地创建 vector。
Vec 有一个形如 iter() 的方法可以为一个 vector 创建迭代器，这允许我们可以轻松地将 vector 用到 for 循环中去。
内存细节：
. Vec 是一个结构体，但是内部其实保存了在堆上固定长度数据的引用。
. 一个 vector 开始有默认大小容量，当更多的元素被添加进来后，它会重新在堆上分配一个新的并具有更大容量的定长列表。（类似 C++ 的 vector）
```
fn main() {
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
```

## 5 所有权和数据借用
### 5.1 所有权
实例化一个类型并且将其绑定到变量名上将会创建一些内存资源，而这些内存资源将会在其整个生命周期中被 Rust 编译器检验。 被绑定的变量即为该资源的所有者。
```
struct Foo {
    x: i32,
}
fn main() {
    // 我们实例化这个结构体并将其绑定到具体的变量上
    // 来创建内存资源
    let foo = Foo { x: 42 };
    // foo 即为该资源的所有者
}
```

### 5.2 基于域的资源管理
Rust 将使用资源最后被使用的位置或者一个函数域的结束来作为资源被析构和释放的地方。 此处析构和释放的概念被称之为 drop（丢弃）。
内存细节：
. Rust 没有垃圾回收机制。
. 在 C++ 中，这被也称为“资源获取即初始化“（RAII）。
```
struct Foo {
    x: i32,
}
fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);
    // foo_a 将在这里被 dropped 因为其在这之后再也没有被使用

    println!("{}", foo_b.x);
    // foo_b 将在这里被 dropped 因为这是函数域的结尾
}
```

### 5.3 释放是分级进行的
删除一个结构体时，结构体本身会先被释放，紧接着才分别释放相应的子结构体并以此类推。
内存细节：
. Rust 通过自动释放内存来帮助确保减少内存泄漏。
. 每个内存资源仅会被释放一次。
```
struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn main() {
    let foo = Foo { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);
    // foo 首先被 dropped 释放
    // 紧接着是 foo.bar
}

```

### 5.4 移交所有权
将所有者作为参数传递给函数时，其所有权将移交至该函数的参数。 在一次移动后，原函数中的变量将无法再被使用。
内存细节:
. 在移动期间，所有者的堆栈值将会被复制到函数调用的参数堆栈中。
```
struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f 在这里被 dropped 释放
}

fn main() {
    let foo = Foo { x: 42 };
    // foo 被移交至 do_something
    do_something(foo);
    // 此后 foo 便无法再被使用
}

```

### 5.5 归还所有权
所有权也可以从一个函数中被归还。
```
struct Foo {
    x: i32,
}
fn do_something() -> Foo {
    Foo { x: 42 }
    // 所有权被移出
}
fn main() {
    let foo = do_something();
    // foo 成为了所有者
    // foo 在函数域结尾被 dropped 释放
}

```

### 5.6 使用引用借用所有权
引用允许我们通过 & 操作符来借用对一个资源的访问权限。 引用也会如同其他资源一样被释放。
```
struct Foo {
    x: i32,
}
fn main() {
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x);
    // f 在这里被 dropped 释放
    // foo 在这里被 dropped 释放
}
```

### 5.7 通过引用借用可变所有权
我们也可以使用 &mut 操作符来借用对一个资源的可变访问权限。 在发生了可变借用后，一个资源的所有者便不可以再次被借用或者修改。
内存细节：
. Rust 之所以要避免同时存在两种可以改变所拥有变量值的方式，是因为此举可能会导致潜在的数据争用（data race）。
```
struct Foo {
    x: i32,
}
fn do_something(f: Foo) {
    println!("{}", f.x);
    // f 在这里被 dropped 释放
}
fn main() {
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
}

```

### 5.8 解引用
使用 &mut 引用时, 你可以通过 * 操作符来修改其指向的值。 你也可以使用 * 操作符来对所拥有的值进行拷贝（前提是该值可以被拷贝——我们将会在后续章节中讨论可拷贝类型）。
```
fn main() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // 取得所有者值的拷贝
    *f = 13;      // 设置引用所有者的值
    println!("{}", bar);
    println!("{}", foo);
}
```

### 5.9 传递借用的数据
Rust 对于引用的规则也许最好用以下的方式总结：
. Rust 只允许同时存在一个可变引用或者多个不可变引用，不允许可变引用和不可变引用同时存在。
. 一个引用永远也不会比它的所有者存活得更久。
而在函数间进行引用的传递时，以上这些通常都不会成为问题。
内存细节：
. 上面的第一条规则避免了数据争用的出现。什么是数据争用？在对数据进行读取的时候，数据争用可能会因为同时存在对数据的写入而产生不同步。这一点往往会出现在多线程编程中。
. 而第二条引用规则则避免了通过引用而错误的访问到不存在的数据（在 C 语言中被称之为悬垂指针）。
```
struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) {
    f.x += 1;
    // 可变引用 f 在这里被 dropped 释放
}

fn main() {
    let mut foo = Foo { x: 42 };
    do_something(&mut foo);
    // 因为所有的可变引用都在 do_something 函数内部被释放了
    // 此时我们便可以再创建一个
    do_something(&mut foo);
    // foo 在这里被 dropped 释放
}

```

### 5.10 引用的引用
引用甚至也可以用在其他引用上。
```
struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x 在这里被 dropped 释放从而允许我们再创建一个不可变引用
    let y = do_something(&foo);
    println!("{}", y);
    // y 在这里被 dropped 释放
    // foo 在这里被 dropped 释放
}

```

### 5.11 显式生命周期
尽管 Rust 不总是在代码中将它展示出来，但编译器会理解每一个变量的生命周期并进行验证以确保一个引用不会有长于其所有者的存在时间。 同时，函数可以通过使用一些符号来参数化函数签名，以帮助界定哪些参数和返回值共享同一生命周期。 生命周期注解总是以 ' 开头，例如 'a，'b 以及 'c。
```
struct Foo {
    x: i32,
}

// 参数 foo 和返回值共享同一生命周期
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x 在这里被 dropped 释放从而允许我们再创建一个不可变引用
    let y = do_something(&foo);
    println!("{}", y);
    // y 在这里被 dropped 释放
    // foo 在这里被 dropped 释放
}

```

### 5.12 多个生命周期
生命周期注解可以通过区分函数签名中不同部分的生命周期，来允许我们显式地明确某些编译器靠自己无法解决的场景。
```
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
fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };
    let x = do_something(&foo_a, &foo_b);
    // foo_a 在这里被 dropped 释放因为只有 foo_b 的生命周期在此之后还在延续
    println!("{}", x);
    // x 在这里被 dropped 释放
    // foo_b 在这里被 dropped 释放
}
```

### 5.13 静态生命周期
一个静态变量是一个在编译期间即被创建并存在于整个程序始末的内存资源。他们必须被明确指定类型。 一个静态生命周期是指一段内存资源无限期地延续到程序结束。需要注意的一点是，在此定义之下，一些静态生命周期的资源也可以在运行时被创建。 拥有静态生命周期的资源会拥有一个特殊的生命周期注解 'static。 'static 资源永远也不会被 drop 释放。 如果静态生命周期资源包含了引用，那么这些引用的生命周期也一定是 'static 的。（任何缺少了此注解的引用都不会达到同样长的存活时间）
内存细节：
. 因为静态变量可以全局性地被任何人访问读取而潜在地引入数据争用，所以修改它具有内在的危险性。我们会在稍后讨论使用全局数据的一些挑战。
. Rust 允许使用 unsafe { ... } 代码块来进行一些无法被编译器担保的内存操作。The R̸͉̟͈͔̄͛̾̇͜U̶͓͖͋̅Ṡ̴͉͇̃̉̀T̵̻̻͔̟͉́͆Ơ̷̥̟̳̓͝N̶̨̼̹̲͛Ö̵̝͉̖̏̾̔M̶̡̠̺̠̐͜Î̷̛͓̣̃̐̏C̸̥̤̭̏͛̎͜O̶̧͚͖͔̊͗̇͠N̸͇̰̏̏̽̃（常见的中文翻译为：Rust 死灵书）在讨论时应该被严肃地看待，
```
static PI: f64 = 3.1415;

fn main() {
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
}
```

### 5.14 数据类型中的生命周期
和函数相同，数据类型也可以用生命周期注解来参数化其成员。 Rust 会验证引用所包含的数据结构永远也不会比引用指向的所有者存活周期更长。 我们不能在运行中拥有一个包括指向虚无的引用结构存在！
```
struct Foo<'a> {
    i:&'a i32
}

fn main() {
    let x = 42;
    let foo = Foo {
        i: &x
    };
    println!("{}",foo.i);
}

```