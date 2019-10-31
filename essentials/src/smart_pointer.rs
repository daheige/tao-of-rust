#[cfg(test)]
mod tests {
    // Rust中的值默认被分配到栈内存，通过Box<T>将值boxing，在堆内存中分配
    // Box<T> 是指向类型为T的堆内存分配值的智能指针
    // 可以通过解引用来获取Box<T> 中的 T
    // Box<T>的行为像引用，并且可以自动释放内存

    // Rust smart pointers: Box<T>, Rc<T>, Ref<T>, RefMut<T>, RefCel<T>

    use crate::smart_pointer::Point;

    #[test]
    fn test_box() {
        let box_point = Box::new(Point { x: 0.0, y: 0.0 });
        println!("{:?}", box_point);
        let unboxed_point = *box_point;
        assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 });
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
