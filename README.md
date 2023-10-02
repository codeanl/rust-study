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