//! generics and trait
/// trait是唯一的接口抽象方式，trait将类型（数据）和行为明确的进行了区分，组合由于继承、面向接口编程
/// ```fn_static<T: traitT>(o: T) {}``` 静态分发，在编译阶段会将实现了trait的类型进行展开，
/// 对编译器来说有多少个类型实现就会有多少个展开，不是泛型的
///
/// ```fn_dyn(o: &traitT) {}``` 动态分发，在运行时查询实现了trait的相应类型，不过开销也很小
///
/// 另开销原则：如果你不使用某个抽象，就不用为它付出开销（静态分发）
/// 如果你确实需要使用该抽象，可以保证这是开销最小的使用方式（动态分发）
///
/// Result<T, E> 是 Option<T> 类型的升级版本
/// ```
/// enum Result<T, E> {
///     Ok(T)
///     Err(E),
/// }
/// ```
///
/// Rust is an expression language, in Rust everything is an expression.
/// 代码即文档，文档即代码。
///
/// Rust编译期对while循环条件不进行求值，只是认为其返回一个() unit类型，
/// 所以在while里面直接返回类型是不被编译期捕捉到的，因为编译器认为while块可能进入也可能不进入
/// 这是因为受到了CTFE功能的限制。如果需要使用无限循环，需要使用loop循环。
///
mod collections;
mod smart_pointer;

pub fn answer() -> () {
    // let创建的变量一般称为绑定binding，
    // 它表明了标识符identifier和值value之间建立的一个关联关系
    // Rust中的表达式分为位置表达式place expression和值表达式value expression，
    // 在其他语言中一般叫做左值LValue和右值RValue

    // 位置表达式就是表示内存位置的表达式，可以对某个数据单元的内存进行读写
    // 值表达式一般只引用了某个存储单元地址中的数据，它相当于数据值，

    // 从语义角度来说，位置表达式代表了持久性数据，
    // 值表达式代表了临时数据

    // 位置表达式默认为不可变绑定，只能对相应的存储单元进行读取
    let a = 40;
    // 使用mut关键字声明可变绑定的位置表达式，可以对相应的存储单元进行写入
    let mut b = 2;

    assert_eq!(sum(a, b), 42);
    // 可以将Rust看作一切皆表达式
    // ()
}

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn show_ownership_move() {
    // place1是一个place expression位置表达式
    let place1 = "hello";
    let place2 = "hello".to_string();
    // 当位置表达式出现在复制操作符右侧，即一个值上下文内，place1会将内存地址转移给other
    let other = place1;
    println!("{:?}", other);
    let other = place2;
    println!("{:?}", other);
}

fn reference() {
    let a = [1, 2, 3];
    // & 操作符将右侧a的值上下文变成了位置上下文
    let b = &a;
    // address
    println!("{:p}", b);
    let mut c = vec![1, 2, 3];
    // 要获取可变引用，必须先声明可变绑定
    let d = &mut c;
    d.push(4);
    println!("{:?}", d); // [1, 2, 3, 4]
    println!("{:?}", c); // [1, 2, 3, 4]
    let e = &42;
    assert_eq!(42, *e);
}

fn fizz_buzz(x: i32) -> String {
    if x % 15 == 0 {
        return String::from("fizzbuzz");
    }
    if x % 3 == 0 {
        return String::from("fizz");
    }
    if x % 5 == 0 {
        return String::from("buzz");
    }

    x.to_string()
}

fn lifetimes() {
    // Rust语言的作用域是静态作用域，即词法作用域（Lexical Scope）
    // 由一对花括号来开辟作用域，词作用域在词法分析阶段就已经确定了，不会动态改变
    let v = "hello world!";
    assert_eq!(v, "hello world!");
    // 使用let定义同名变量的做法叫做变量遮蔽variable shadow
    let v = "hello Rust!";
    assert_eq!(v, "hello Rust!");
    {
        let v = "hello world!";
        assert_eq!(v, "hello world!");
        // 从使用 let 声明创建变量绑定开始，到超出词法作用域的范围时结束
    }
    assert_eq!(v, "hello Rust!");
    // Rust还有NNL(Non Lexical Lifetimes)，非词法声明周期，结合上下文使用情况检查生命周期是否产生越权
    // 如果没有则可以编译通过，而不只是严格的按照词法分析一刀切，提升所有权灵活性
}

// 函数指针
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn is_true() -> bool {
    true
}

fn true_maker() -> fn() -> bool {
    is_true
}

fn main() {
    answer();
    show_ownership_move();
    reference();
    assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
    assert_eq!(fizz_buzz(3), "fizz".to_string());
    assert_eq!(fizz_buzz(5), "buzz".to_string());
    assert_eq!(fizz_buzz(13), "13".to_string());
    lifetimes();

    let a = 2;
    let b = 3;
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);

    assert_eq!(true_maker()(), true);

    // 数组初始化必须在编译期就知道长度
    let arr = [0; init_len(3)];
    assert_eq!(arr.len(), 3 * 2);
    let arr = [0; init_len(6)];
    assert_eq!(arr.len(), 6 * 2);

    // closure can capture environment vars which function can not.
    let out = 42;
    fn add(i: i32, j: i32) -> i32 {
        i + j
    }
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    let closure_inferred = |i, j| i + j + out;
    let (i, j) = (1, 2);
    assert_eq!(3, add(i, j));
    assert_eq!(45, closure_annotated(i, j));
    assert_eq!(45, closure_inferred(i, j));

    let (a, b) = (2, 3);
    assert_eq!(closure_math(|| a + b), 5);
    assert_eq!(closure_math(|| a * b), 6);

    let result = two_times_impl();
    assert_eq!(result(3), 6);

    flow_control();
    match_expr();
    primitive();
}

fn primitive() {
    // bool
    let x = true;
    let y = false;
    assert_eq!(x as i32, 1);
    assert_eq!(y as i32, 0);

    // 基本数字类型：
    // 固定大小的类型：无符号整数 u8, u16, u32, u64, u128、有符号整数 i8, i16, i32, i64, i128
    // 动态大小的类型：usize/isize: 占用4个字节或8个字节，取决于机器的字长
    // 浮点数：f32, f64
    let num = 42u32;
    let num = 42;
    let num = 0x2A; // 16进制
    let num = 0o106; // 8进制
    let num = 0b1101_1011; // 二进制
    assert_eq!(b'*', 42u8); // 字节字面量
    assert_eq!(b'\'', 39u8);
    assert_eq!(b'\'' as u32, 39);
    let num = 3.14159f64;
    assert_eq!(-3.14, -3.14f64);
    assert_eq!(2., 2.0f64);
    assert_eq!(2e4, 20000f64);
    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MIN);
    println!("{:?}", std::f32::MAX);

    // char
    // Unicode标量值 每个字符占4个字节
    let x = 'r';
    let x = 'r';
    println!("{}", '\'');
    println!("{}", '\\');
    println!("{}", '\n');
    println!("{}", '\r');
    println!("{}", '\t');
    // 使用ASCII码来定义字符
    // \x2A 是十六进制表示，格式为：\xHH
    assert_eq!('\x2A', '*');
    // \u{HH} 是Unicode十六进制码
    println!("{}", '\u{151}');
    assert_eq!('%' as i8, 37);

    // array [T; N]
    // 数组大小固定、元素均为同类型;
    // 对于原始固定长度数组，只有impl Copy的T才能作为其元素，也就是说这些元素都存在栈上
    let arr: [i32; 3] = [1, 2, 3];
    let mut arr = [1, 2, 3];
    assert_eq!(1, arr[0]);
    arr[0] = 3;
    assert_eq!(3, arr[0]);
    let arr = [0; 10];
    assert_eq!(arr.len(), 10);
    assert_eq!(0, arr[5]);

    // range
    assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    assert_eq!(3 + 4 + 5, (3..6).sum());
    assert_eq!(3 + 4 + 5 + 6, (3..=6).sum());
    for i in (1..5) {
        println!("{}", i);
    }
    for i in 1..=5 {
        println!("{}", i);
    }

    // array
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    assert_eq!(&arr[1..], &[2, 3, 4, 5]);
    assert_eq!(arr[1..], [2, 3, 4, 5]);
    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    assert_eq!(arr, &[1, 7, 3]);
    let vec = vec![1, 2, 3];
    assert_eq!(vec[..], [1, 2, 3]);
    assert_eq!(&vec[..], [1, 2, 3]);

    // str String
    // 处于内存安全的考虑，Rust将字符串分为两种类型，
    // 一种是固定长度字符串，即str，通常以不可变借用的形式存在，&str，str硬编码到二进制文件中，&str是对二进制硬编码位置的地址引用
    // 另一种是可变字符串，可随意改变其长度，就是String，String分配在堆上
    let truth: &'static str = "Rust is a graceful language";
    let ptr = truth.as_ptr();
    let len = truth.len();
    assert_eq!(27, len);
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    assert_eq!(s, Ok(truth));

    // pointers: 表示内存地址的类型
    // reference: 引用，本质是一种非空指针
    // raw pointer: 原生指针，主要用于unsafe rust，分为 不可变原生指针 *const T 和可变原生指针 *mut T
    // fn pointer: 函数指针
    // smart pointer: 智能指针
    let mut x = 10;
    let ptr_x = &mut x;
    println!("&mut x: {:?}", ptr_x); // 10
    let ptr_x = &mut x as *mut i32; // mutable raw pointer
    println!("*mut i32: {:?}", ptr_x); // 0x7ffee392cf7c
    let y = Box::new(20); // alloc in heap
    let ptr_y = &*y;
    let ptr_y = ptr_y as *const i32; // immutable raw pointer
    unsafe {
        // raw pointer need to be cover by unsafe block
        *ptr_x += *ptr_y;
    }
    assert_eq!(x, 30);

    // never type
    let num: Option<u32> = Some(42u32);
    match num {
        Some(n) => n,
        // match表达式要求所有分支都必须返回相同类型，而panic!宏其实返回的是never type，
        // 编译器没有报错是因为never type可以强制转换为其他任何类型
        None => panic!("Nothing"),
    };

    // 复合数据类型：
    // tuple: 元组
    // struct: 结构体：具名结构体 Named-Field Struct，元组结构体 Tuple-Link Struct，单元结构体 Unit-Like Struct
    // enum: 枚举
    // union: 联合体
    let tuple = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    let coords = (0, 1);
    let result = move_coords(coords);
    assert_eq!(result, (1, 2));
    // let match pattern, destructure tuple
    let (x, y) = result;
    assert_eq!(x, 1);

    let alex = People::new("Alex", 1);
    // 面对对象消息通信模型receiver.message
    println!("name: {:?}", alex.name());
    alex.gender();

    let color = Color(10, 20, 30);
    assert_eq!(color.0, 10);
    assert_eq!(color.1, 20);
    assert_eq!(color.2, 30);

    let int = Integer(10);
    assert_eq!(int.0, 10);
    let int: Int = 20;
    assert_eq!(int, 20);

    let x = Empty;
    println!("{:p}", &x);
    let y = x;
    println!("{:p}", &y);
    let z = Empty;
    println!("{:p}", &z);
    assert_eq!((..), std::ops::RangeFull);

    let x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4; // 函数指针
    let home = IpAddr::V4(127, 0, 0, 1);
}

enum Number {
    Zero,
    One,
    Two,
}

// c-like enum
enum Color1 {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum IpAddr {
    // 携带了参数的枚举值，本质上属于函数指针类型
    V4(u8, u8, u8, u8),
    V6(String),
}

enum OptionInt {
    Some(i32),
    None,
}

// Tuple-Like struct, file no named
struct Color(i32, i32, i32);
// 当元组结构体只有一个字段的时候，我们称之为New Type模式
struct Integer(u32);
// 可以使用type关键字为一个类型创建别名
type Int = i32;
// Rust中可以定义一个没有任何字段的结构体：单元结构体
struct Empty;
struct Empty1 {}

// Named-Field struct
#[derive(Debug, PartialEq)]
struct People {
    name: &'static str,
    gender: u32,
}

impl People {
    fn new(name: &'static str, gender: u32) -> Self {
        People { name, gender }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn gender(&self) {
        let gender = if self.gender == 1 { "boy" } else { "girl" };
        println!("gender: {:?}", gender);
    }
}

fn move_coords(x: (i32, i32)) -> (i32, i32) {
    (x.0 + 1, x.1 + 1)
}

// `never` type, 用于表示永远不可能有返回值的计算类型，比如线程退出的时候就不会有返回值
// never type 是一个试验特性
//#[feature(never_type)]
//fn foo() -> u32 {
//    let x: ! = { return 123 };
//}

fn match_expr() {
    let number = 42;
    match number {
        0 => println!("Origin"),
        1..=3 => println!("All"),
        1 | 5 | 7 | 13 => println!("Bad Luck"),
        // 使用 @ 可以将分支的值绑定到变量，供分支右侧的代码使用，这类匹配叫做绑定模式 binding mode
        n @ 42 => println!("Answer is {}", n),
        _ => println!("Common"),
    }

    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        binary = 1;
    }
    assert_eq!(binary, 1);

    let mut v = vec![1, 2, 3, 4, 5];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

//fn while_true(x: i32) -> i32 {
//    // con not compile, Compile-Time Function Execution not support while/if block.
//    // so rustc only know while block return unit (), not the condition and its result return x + 1;
//    while true {
//        return x + 1;
//    }
//}

// flow control expression
fn flow_control() {
    let n = 13;
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    assert_eq!(big_n, 6);

    for i in 1..=3 {
        print!("{} ", i);
    }

    let mut i = 0;
    while i < 3 {
        print!("{} ", i);
        i += 1;
    }

    let mut i = 0;
    // 无限循环推荐使用loop而不是while(true)
    loop {
        print!("{} ", i);
        i += 1;
        if i >= 3 {
            break;
        }
    }
    println!();
}

// 使用了 impl Fn(i32) -> i32 作为返回类型，它表示实现实现了Fn(i32) -> i32的类型
// 闭包的实现背后是一个隐式的结构体和一个trait，这里可以看成这个trait就是Fn(i32) -> i32
fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    // 如果返回需要用到函数内的ownership，则须要move进行转移
    move |j| j * i
}

fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

// CTFE(Compile-Time Function Execution)，编译时函数执行
// using CTFE we can determine array size dynamic
// const关键字一般用于定义全局常量，const fn可以强制编译器在编译器执行函数
const fn init_len(n: usize) -> usize {
    return n * 2;
}

// trait Foo {}
// const generics is experimental
// impl<T, const N: usize> Foo for [T; N] {}
