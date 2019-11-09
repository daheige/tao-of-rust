//! trait是Rust的灵魂。
//! Rust中所有的抽象（接口抽象、OOP范式抽象、函数式抽象等），都是基于trait来完成的
//! 同时trait也保证了这些抽象几乎都是运行时零开销的
//!
//! 从类型系统角度来说，trait是Rust对Ad-hoc多态的支持
//! 从语义上来说，trait是在行为上对类型的约束：
//! 1. 接口抽象：对类型行为的统一约束
//! 2. 泛型约束：有限的范围内
//! 3. 抽象类型：动态分发
//! 4. 标签trait：对类型的约束，可理解为 `enum Type;` ?

#[cfg(test)]
mod tests {
    trait Add<RHS, Output> {
        fn add(self, rhs: RHS) -> Output;
    }
    impl Add<i32, i32> for i32 {
        fn add(self, rhs: i32) -> i32 {
            self + rhs
        }
    }
    impl Add<u32, i32> for u32 {
        fn add(self, rhs: u32) -> i32 {
            (self + rhs) as i32
        }
    }
    /// 为类型实现trait的时候Rust遵循一条重要的规则：孤儿规则（Orphan Rule）
    /// 即类型或者trait必须至少有一个在当前scope定义，防止覆盖不在当前scope定义的类型和trait的实现行为
    impl Add<u64, u64> for u32 {
        fn add(self, rhs: u64) -> u64 {
            (self as u64) + rhs
        }
    }

    // RSH default value is Self, Self是每个trait都带有的隐式类型参数，代表当前trait的具体类型
    trait Add2<RHS = Self> {
        /// 关联类型
        type Output;
        fn add(self, rhs: RHS) -> Self::Output;
    }

    #[test]
    fn test_trait() {
        let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
        let x = a.add(b);
        let y = c.add(d);
        assert_eq!(x, 3i32);
        assert_eq!(y, 7i32);

        let a = "hello";
        let b = " world";
        let c = a.to_string() + b;
        println!("{:?}", c);

        let a = 1u32;
        let b = 2u64;
        assert_eq!(a.add(b), 3u64);
    }

    /// Rust不支持传统面向对象的继承，而是支持trait继承，使用trait继承可以减少重复简化编程，方便组合
    trait Page {
        fn set_page(&self, p: i32) {
            println!("Page Default: 1");
        }
    }
    trait PerPage {
        fn set_perpage(&self, num: i32) {
            println!("Per Page Default: 10");
        }
    }
    struct MyPaginate {
        page: i32,
    }
    impl Page for MyPaginate {}
    impl PerPage for MyPaginate {}

    #[test]
    fn test_trait_not_inherit() {
        let my_paginate = MyPaginate { page: 1 };
        my_paginate.set_page(2);
        my_paginate.set_perpage(100);
    }

    // trait inherit
    trait Paginate: Page + PerPage {
        fn set_skip_page(&self, num: i32) {
            println!("Skip Page: {:?}", num);
        }
    }
    /// 为泛型T实现Paginate，空快表示直接使用Paginate的默认实现
    /// 其中泛型T的范围限定是Page+PerPage，即在实现了Page+PerPage的类型的基础上，再为它实现Paginate
    /// 而此处Paginate又继承自Page+PerPage，即表示在Page+PerPage的基础上，添加了Paginate的新增的默认实现
    /// 这样可以对原来已实现了Page+PerPage的类型在不侵入的情况下，新增另一个trait的实现！！
    /// trait继承也可以用于扩展标准库中的方法
    /// 包含trait bound的generics是静态分发
    impl<T: Page + PerPage> Paginate for T {
        //        fn set_skip_page(&self, num: i32) {
        //            unimplemented!()
        //        }
    }

    #[test]
    fn test_trait_inherit() {
        let my_paginate = MyPaginate { page: 1 };
        my_paginate.set_page(1);
        my_paginate.set_perpage(100);
        my_paginate.set_skip_page(12);
    }

    /// 理解类型限定：编程语言中的Structure Typing，结构化类型，用来判断类型是否等价
    /// 从数学角度看，类型可以看做具有相同属性值的集合，trait也是一种类型（Everything is type in Rust），
    /// 是一种方法/行为的集合, Paginate: Page + PerPage，在韦恩图中就是表示，Paginate是Page和PerPage的交集
    /// Rust编程的哲学是组合由于继承，Rust并不提供数据类型层面上的继承（即传统面向对象的继承）
    /// Rust中的数据类型是语言允许的最小集合，而trait限定可以对这些类型集合进行组合（求交集）
    fn sum<T>(a: T, b: T) -> T
    where
        T: std::ops::Add<T, Output = T>, // 这里的Add<T, Output = T>中的RHS=T可以省略，缺省为和Self同类型
                                         // 变成Add<Output=T>
    {
        a + b
    }

    #[test]
    fn test_trait_bound() {
        assert_eq!(sum(1u32, 2u32), 3u32);
        assert_eq!(sum(1u64, 2u64), 3);
    }

    /// trait还可以用作抽象类型（Abstract Type），也叫存在类型（Existential Type）
    /// 相对于具体类型，抽象类型不能直接实例化，对于抽象类型，编译器无法确定其确切的功能和所占的空间大小
    #[derive(Debug)]
    struct Foo;
    trait Bar {
        fn baz(&self);
    }
    impl Bar for Foo {
        fn baz(&self) {
            println!("{:?}", self);
        }
    }
    /// trait本身也是一种类型，但是它的类型大小在编译器是无法确定的，所以trait对象必须使用指针
    /// 可以利用操作符&或Box<T>来制造一个trait对象，大概类似下面：
    /// ``` 以impl MyTrait for T为例：
    /// pub struct TraitObject {
    ///     pub data: *mut (),   // data指针，指向trait对象保存的类型数据T
    ///     pub vtable: *mut (), // vtable指针指向包含为T实现的MyTrait的Vtable(Virtual Table，虚表)
    ///                          // 虚表的本质是一个结构体，包含了析构函数、大小、对齐和方法等信息
    /// }
    /// ```
    /// 在编译器，编译器只知道TraitObject包含指针的信息，并且指针的大小也是确定的，并不知道要调用哪个方法
    /// 在运行期，当有trait_object.method()被调用时，TraitObject会根据vtable的指针从虚表中查出正确的方法指针，
    /// 然后再进行动态调用，这就是动态分发。
    ///
    /// ```trait对象的安全问题```
    /// 并不是每个trait都可以作为trait对象被使用，这和类型大小是否确定有关系
    /// 每个trait都包含了一个隐式的类型参数Self，代表实现该trait的类型
    /// 其中Self默认有一个隐式的trait bound: ?Sized，形如<Self: ?Sized>，
    /// ?Size trait包括了所有的动态大小类型和所有可确定大小的类型
    /// Rust中大部分类型都默认是可确定的类型，也就是<T：Sized>，这也是泛型代码可以正常编译的原因
    /// 当trait对象在运行期进行动态分发时，也必须确定大小，否则无法为其正确分配内存空间
    /// 所以必须同时满足以下两条规则的trait才可以作为trait对象使用：
    /// + trait的Self类型参数不能被限定为Sized
    /// + trait中所有的方法都必须是对象安全的（）
    /// 满足这2条规则的trait就是对象安全的trait
    /// ```对象安全```
    /// ```
    /// trait Foo: Sized { // Foo继承自Sized，这表明要为某类型实现Foo必须先实现Sized
    ///     fn some_method(&self); // 所以Foo中的隐式Self也必然是Sized的
    /// }
    /// ```
    /// trait对象本身是动态分发的，编译器根本无法确定Self具体是哪个类型，
    /// 因为不指定给哪些类型实现过该trait，更无法确定其大小，而现在又需要限定Foo: Sized，所以Foo不是对象安全的
    /// 对象安全的本质就是为了让trait对象可以安全地调用相应的方法，如果给trait加上Self: Sized限定，
    /// 那么在动态调用trait对象的过程中，如果遇到了Unsized类型，可能引发段错误（segment fault）。
    /// 当把trait当做对象使用时，其内部类型就默认为Unsized类型，也就是动态大小类型，
    /// 只是将其置于编译器可确定大小的胖指针背后，以供运行时动态调用
    ///
    fn static_dispatch<T>(t: &T)
    where
        T: Bar,
    {
        t.baz();
    }
    fn dynamic_dispatch(t: &Bar) {
        t.baz();
    }

    #[test]
    fn test_trait_dispatch() {
        let foo = Foo;
        static_dispatch(&foo);
        dynamic_dispatch(&foo);
    }
}
