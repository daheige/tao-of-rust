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
}
