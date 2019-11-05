//! 在类型系统中，一切皆类型。
//! 基于类型定义的一系列组合、运算和转换等方法，可以看做类型的行为。
//! Rust属于显式静态类型，一切且表达式，表达式皆有值，值皆有类型，一切皆类型。
//!
//! 除基本的原生类型和复合类型，Rust把作用域scope也纳入了类型系统：生命周期。
//! 一些无法返回值的情况也都被纳入了类型系统，如线程奔溃、break或continue等行为，这种类型叫做never类型
//! Rust的类型系统囊括了编程中遇到的所有情况，一般不会出现未定义的行为，所以说，Rust是类型安全的语言。
//!
//! 编程语言中不同的类型本质上是内存占用空间和编码方式的不同，Rust代码被编译为LLVM IR，其中携带了内存分配的信息，
//! 所以编译器需要事先知道类型的大小才能分配合理的内存。
//! 可确定大小类型（Sized Type）：大部分类型都是在编译器可确定大小的类型
//! 动态大小类型（Dynamic Sized Type, DST）：少量的动态大小的类型，比如str类型的字符串字面量，对于编译器来说，
//! str类型的大小是无法确定的，对于这种情况，Rust提供了引用类型，因为引用总会有固定的且在编译器已知的大小
//! 零大小类型（Zero Sized Type, ZST）：比如 单元类型() 和 单元结构体struct T;
//! 由零大小类型构成的数组大小也为零，零大小类型的特点是，它们的值就是其本身，运行时并不占用内存空间
//! （直接被编译到二进制文件中作为字面量使用？）
//!
//! 参数化多态 -> 泛型
//! Ad-hoc多态 -> 特定多态，同一种行为定义，在不同的上下文中会响应不同的行为，如trait
//! 子类型多态 -> 面向对象语言中继承的子类型的多态，如Java
//!
//! Rust支持泛型和trait，即参数化多态和Ad-hoc多态

fn main() {
    // 包含了动态大小类型信息和携带了长度信息的指针，叫做胖指针（Fat Pointer）, &str是一种胖指针
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    println!("{:p}", ptr);
    println!("{:?}", str.len());

    // [u32] 在编译器是大小不确定的，编译器无法得知其类型长度，编译不过；
    // [u32; 5] 使用具体长度的类型，可通过编译
    // &[u32] 使用引用类型，使得其类型长度是固定的，可通过编译，&[u32]是对[u32]的引用，胖指针，带了长度信息
    // [u32], [u32; 5] 和 &[u32] 都是不同的类型
    let mut arr = [1 as u32, 2, 3, 4, 5];
    println!("{:?}", arr);
    reset(&mut arr);
    println!("{:?}", arr);
    // 普通指针，占8个字节
    assert_eq!(std::mem::size_of::<&[u32; 5]>(), 8);
    // 胖指针，占16个字节
    assert_eq!(std::mem::size_of::<&mut [u32]>(), 16);

    assert_eq!(std::mem::size_of::<()>(), 0);
    assert_eq!(std::mem::size_of::<Foo>(), 0);
    assert_eq!(std::mem::size_of::<Void>(), 0);
    assert_eq!(std::mem::size_of::<Baz>(), 0);
    assert_eq!(std::mem::size_of::<[(); 10]>(), 0);
    assert_eq!(std::mem::size_of::<[Void; 10]>(), 0);
    assert_eq!(std::mem::size_of::<[Foo; 10]>(), 0);
    assert_eq!(std::mem::size_of::<[Baz; 10]>(), 0);

    for i in vec![(); 10] {
        print!("{:?}", i);
    }
    vec![(); 20].into_iter().for_each(|i| print!("{:?}", i));
    // 在一些需要迭代次数的场合中，使用vec![(); 10]这种方式能获得较高的性能，
    // 因为Vec内部迭代器会针对ZST类型做一些优化
}

fn reset(arr: &mut [u32]) {
    for i in 0..5 {
        arr[i] = (5 - i) as u32;
    }
}

enum Void {}
struct Foo;
struct Baz {
    foo: Foo,
    qux: (),
    bza: [u8; 0],
}
