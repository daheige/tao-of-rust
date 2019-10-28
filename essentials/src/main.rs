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
}

fn match_expr() {
    let number = 42;
    match number {
        0 => println!("Origin"),
        1...3 => println!("All"),
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
