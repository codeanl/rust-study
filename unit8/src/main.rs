fn main() {
    // 2
    let a = 42;
    let memory_location = &a as *const i32 as usize;
    println!("Data is here {}", memory_location);

    // 4
    let a: i32 = 42;
    let ref_ref_ref_a: &&&i32 = &&&a;
    let ref_a: &i32 = **ref_ref_ref_a;
    let b: i32 = *ref_a;
    println!("{}", b)

    // 5
    let f = Foo { value: 42 };
    let ref_ref_ref_f = &&&f;
    println!("{}", ref_ref_ref_f.value);

    // 6
    let foo = TattleTell {
        value: "secret message",
    };
    // dereference occurs here immediately 
    // after foo is auto-referenced for the
    // function `len`
    println!("{}", foo.len());

    // 7
    let a: [u8; 4] = [86, 14, 73, 64];
    // this is a raw pointer. Getting the memory address
    // of something as a number is totally safe
    let pointer_a = &a as *const u8 as usize;
    println!("Data memory location: {}", pointer_a);
    // Turning our number into a raw pointer to a f32 is
    // also safe to do.
    let pointer_b = pointer_a as *const f32;
    let b = unsafe {
        // This is unsafe because we are telling the compiler
        // to assume our pointer is a valid f32 and
        // dereference it's value into the variable b.
        // Rust has no way to verify this assumption is true.
        *pointer_b
    };
    println!("I swear this is a pie! {}", b);


    // 8
    let p = Pie::new();
    // "make a pie" by dereferencing our 
    // Pie struct smart pointer
    println!("{:?}", *p);

    // 9
    let heap_pie = Box::new(Pie);
    heap_pie.eat();
}

// 5
struct Foo {
    value: i32
}

// 6
use std::ops::Deref;
struct TattleTell<T> {
    value: T,
}
impl<T> Deref for TattleTell<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("{} was used!", std::any::type_name::<T>());
        &self.value
    }
}

// 8
use std::alloc::{alloc, Layout};
use std::ops::Deref;

struct Pie {
    secret_recipe: usize,
}

impl Pie {
    fn new() -> Self {
        // let's ask for 4 bytes
        let layout = Layout::from_size_align(4, 1).unwrap();

        unsafe {
            // allocate and save the memory location as a number
            let ptr = alloc(layout) as *mut u8;
            // use pointer math and write a few 
            // u8 values to memory
            ptr.write(86);
            ptr.add(1).write(14);
            ptr.add(2).write(73);
            ptr.add(3).write(64);

            Pie { secret_recipe: ptr as usize }
        }
    }
}
impl Deref for Pie {
    type Target = f32;
    fn deref(&self) -> &f32 {
        // interpret secret_recipe pointer as a f32 raw pointer
        let pointer = self.secret_recipe as *const f32;
        // dereference it into a return value &f32
        unsafe { &*pointer }
    }
}

// 9
struct Pie;
impl Pie {
    fn eat(&self) {
        println!("tastes better on the heap!")
    }
}
