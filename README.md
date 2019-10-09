### Rust设计哲学

1.内存安全
    类型安全
    所有权系统
    借用和生命周期
2.零成本抽象
3.实用性


### Rust语言架构

混合编程范式：面向对象 + 函数式
语义：所有权 + MOVE + COPY + 借用 + 生命周期 + DROP
类型系统：泛型 + Trait + 一切皆类型 + 多态 + 类型推断
安全内存管理 + 栈 + RAII + 堆

### Rust代码如何执行
Rust代码 -> AST抽象语法树 -> HIR(High-level IR) -> MIR(Middle IR) -> LLVM IR -> LLVM -> Machine Code

未来的互联网：安全 + 高性能

### Rust语言的基本构成
+ 语言规范
    Rust语言参考（The Rust Reference
    RFC文档
+ 编译器
    rustc是用Rust语言开发的（从Rust实现自举之后？
+ 核心库
    Rust的语法由核心库和标准库共同提供，其中Rust核心库是标准库的基础
    核心库定义的是Rust的核心，不依赖于操作系统和网络相关的库，甚至不知道堆分配，也不提供并发和I/O
    做嵌入式开发时核心库是必须的
+ 标准库
    标准库提供应用程序开发所需要的基础和跨平台支持，包含内容如下：
    - 与核心库一样的基本trait、原始数据类型、功能数据类型和常用宏等，以及与核心库几乎完全一致的API
    - 并发、I/O和运行时，例如线程模块、通道、Sync trait等并发模块，以及文件、TCP、UDP、pipe、socket等常见I/O
    - 平台抽象，包括程序参数、环境变量、目录导航
    - 底层操作接口，如 std::mem, std::ptr, std::intrinsics等，操作内存、指针、调用编译器函数
    - 可选和错误处理类型 Option, Result，以及iterator等
+ 包管理器（crate