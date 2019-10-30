// 线性队列：向量 Vec，双端队列 VecQueue，链表 LinkedList
// K-V映射表：无需哈希表 HashMap, 有序哈希表 BTreeMap
// 集合类型：无序集合 HashSet, 有序集合 BTreeSet
// 优先队列：二叉堆 BinaryHeap

#[cfg(test)]
mod tests {
    use std::collections::{LinkedList, VecDeque};

    #[test]
    fn test_vec() {
        let mut v1 = vec![];
        v1.push(1);
        v1.push(2);
        v1.push(3);
        assert_eq!(v1, [1, 2, 3]);
        assert_eq!(v1[1], 2);
        let mut v2 = vec![0; 10];
        let mut v3 = Vec::new();
        v3.push(4);
        v3.push(5);
        v3.push(6);
        assert_eq!(v3.get(4), None); // index out of bounds
    }

    #[test]
    fn test_double_ended_queue() {
        // FIFO + FILO VecQueue是基于可增长的RingBuffer实现的双端队列
        let mut buf = VecDeque::new();
        buf.push_front(1);
        buf.push_front(2);
        assert_eq!(buf.get(0), Some(&2));
        assert_eq!(buf.get(1), Some(&1));
        buf.push_back(3);
        buf.push_back(4);
        buf.push_back(5);
        assert_eq!(buf.get(2), Some(&3));
        assert_eq!(buf.get(3), Some(&4));
    }

    #[test]
    fn test_linked_list() {
        // Rust LinkedList 是双向链表，但是通常最好使用Vec或者VecDeque类型，
        // 因为基于数组的列表比链表更加快速，内存访问效率更高，并且可以更好的利用CPU缓存
        let mut list1 = LinkedList::new();
        list1.push_back('a');
        let mut list2 = LinkedList::new();
        list2.push_back('b');
        list2.push_back('c');
        list1.append(&mut list2);
        println!("{:?}", list1);
        println!("{:?}", list2);
        assert_eq!(list1.pop_front(), Some('a'));
        list1.push_front('e');
        list2.push_front('f');
        assert_eq!(list1.pop_back(), Some('c'));
    }
}
