### Rust设计哲学

1. 内存安全  
   + 类型安全  
   + 所有权系统  
   + 借用和生命周期  

2. 零成本抽象  
3. 实用性


### Rust语言架构

1. 混合编程范式：面向对象 + 函数式  
2. 语义：所有权 + MOVE + COPY + 借用 + 生命周期 + DROP
3. 类型系统：泛型 + Trait + 一切皆类型 + 多态 + 类型推断
4. 安全内存管理 + 栈 + RAII + 堆

### Rust代码如何执行

    Rust代码 -> AST抽象语法树 ->  
    HIR(High-level IR) -> MIR(Middle IR) -> LLVM IR ->  
    LLVM -> Machine Code

未来的互联网：安全 + 高性能

### Rust语言的基本构成
1. 语言规范
   1. Rust语言参考（The Rust Reference
   2. RFC文档
2. 编译器  
    rustc是用Rust语言开发的（从Rust实现自举之后）
3. 核心库  
   1. Rust的语法由核心库和标准库共同提供，其中Rust核心库是标准库的基础  
   2. 核心库定义的是Rust的核心，不依赖于操作系统和网络相关的库，甚至不知道堆分配，也不提供并发和I/O
   3. 做嵌入式开发时核心库是必须的
4. 标准库  
   标准库提供应用程序开发所需要的基础和跨平台支持，包含内容如下：
    + 与核心库一样的基本trait、原始数据类型、功能数据类型和常用宏等，以及与核心库几乎完全一致的API
    + 并发、I/O和运行时，例如线程模块、通道、Sync trait等并发模块，以及文件、TCP、UDP、pipe、socket等常见I/O
    + 平台抽象，包括程序参数、环境变量、目录导航
    + 底层操作接口，如 std::mem, std::ptr, std::intrinsics等，操作内存、指针、调用编译器函数
    + 可选和错误处理类型 Option, Result，以及iterator等
5. 包管理器（cargo）

### Rust索引

#### 入门篇

1. [《Rust 程序设计语言》（The Rust Programming Language）](https://doc.rust-lang.org/book/)
2. [《通过例子学 Rust》（Rust by Example）](https://doc.rust-lang.org/stable/rust-by-example/)
3. [《RustPrimer初级教程》](https://rustcc.gitbooks.io/rustprimer)
4. [《Rust编程之道》](https://book.douban.com/subject/30418895/)
5. [《深入浅出Rust》](https://book.douban.com/subject/30312231/)

#### 起步篇

1. [《The Little Book of Rust Macros》](https://danielkeep.github.io/tlborm/book/index.html)
2. [《Rust宏小册 中文版》](http://blog.luxko.site/tlborm-chinese/book/README.html)
3. [《The Rustonomicon》（The Dark Arts of Unsafe Rust）](https://doc.rust-lang.org/nomicon/)
4. [《Asynchronous Programming in Rust》](https://rust-lang.github.io/async-book/)

#### 工程篇

1. [《Cargo 帮助文档》](https://doc.rust-lang.org/cargo/index.html)
2. [《rustdoc 帮助文档》](https://doc.rust-lang.org/rustdoc/index.html)
3. [《rustc 帮助文档》](https://doc.rust-lang.org/rustc/index.html)
4. [《Rust 编译错误索引》](https://doc.rust-lang.org/error-index.html)

#### 参考篇

1. [《Rust 语言参考》（The Rust Reference）](https://doc.rust-lang.org/reference/index.html)
2. [《Rust RFCs》](https://rust-lang.github.io/rfcs/)
3. [《Rust 版本指南》（The Edition Guide）](https://doc.rust-lang.org/edition-guide/index.html)
4. [Rust Language Cheat Sheet](https://cheats.rs/)

#### 频道篇

1. [The Rust Programming Fortum](https://users.rust-lang.org/)
2. [Rust语言中文社区](https://rust.cc/)
3. [Rust Force CN](https://rustforce.net/)
4. [This Week in Rust](https://this-week-in-rust.org/)
5. [Baby Steps (Nicholas D. Matsakis)](http://smallcultfollowing.com/babysteps/)