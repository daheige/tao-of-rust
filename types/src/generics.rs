#[cfg(test)]
mod tests {
    use crate::Baz;

    fn foo<T>(x: T) -> T {
        x
    }

    #[test]
    fn generics_fn() {
        assert_eq!(foo(1), 1);
        assert_eq!(foo("hello"), "hello");
        println!("12")
    }

    /// Rust 中的泛型属于静多态，编译器会基于使用到的类型进行单态化展开（Monomorphization）
    /// 单态化分发的好处是性能好，没有运行时开销，缺点是容易造成编译后的二进制文件过大
    #[derive(Debug, PartialEq)]
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
    }

    #[test]
    fn generics_struct() {
        let point1 = Point::new(1, 2);
        let point2 = Point::new("1", "2");
        assert_eq!(point1, Point::new(1, 2));
        assert_eq!(point2, Point::new("1", "2"));
    }

    #[derive(Debug, PartialEq)]
    struct Foo(i32);
    #[derive(Debug, PartialEq)]
    struct Bar(i32, i32);
    trait Inst {
        fn new(i: i32) -> Self;
    }
    impl Inst for Foo {
        fn new(i: i32) -> Self {
            Foo(i)
        }
    }
    impl Inst for Bar {
        fn new(i: i32) -> Self {
            Bar(i, i + 10)
        }
    }
    fn foobar<T: Inst>(i: i32) -> T {
        Inst::new(i)
    }

    #[test]
    fn duck_type_generics() {
        let f: Foo = foobar(10);
        assert_eq!(f, Foo(10));
        let b: Bar = foobar(20);
        assert_eq!(b, Bar(20, 30));
        let f = foobar::<Foo>(10);
        assert_eq!(f, Foo(10));
        assert_eq!(foobar::<Bar>(10), Bar(10, 20));
    }
}
