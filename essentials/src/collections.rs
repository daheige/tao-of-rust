// 线性队列：向量 Vec，双端队列 VecQueue，链表 LinkedList
// K-V映射表：无需哈希表 HashMap, 有序哈希表 BTreeMap
// 集合类型：无序集合 HashSet, 有序集合 BTreeSet
// 优先队列：二叉堆 BinaryHeap

#[cfg(test)]
mod tests {
    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };

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

    #[test]
    fn test_kv() {
        // HashMap/BTreeMap<K: Hash, V: Sized> K需要是可哈希的，V需要是在编译器已知大小的
        let mut hmap = HashMap::new();
        hmap.insert(3, "c");
        hmap.insert(1, "a");
        hmap.insert(2, "b");
        hmap.insert(5, "e");
        hmap.insert(4, "d");
        println!("{:?}", hmap);

        let mut bmap = BTreeMap::new();
        bmap.insert(3, "c");
        bmap.insert(2, "b");
        bmap.insert(1, "a");
        bmap.insert(5, "e");
        bmap.insert(4, "d");
        println!("{:?}", bmap);
    }

    #[test]
    fn test_set() {
        // HashSet<K>/BTreeSet<K> 其实就是 HashMap<K,V>和BTreeMap<K,V>把V设置为空元祖的特定类型
        // HashSet<K> => HashSet<K,()>
        let mut hbooks = HashSet::new();
        hbooks.insert("A song of Ice and Fire");
        hbooks.insert("The Emerald City");
        hbooks.insert("The Odyssey");
        if !hbooks.contains("The Emerald City") {
            println!(
                "We have {} bookes, but The Emerald City ain't one.",
                hbooks.len()
            );
        }
        println!("{:?}", hbooks);

        let mut bbooks = BTreeSet::new();
        bbooks.insert("A song of Ice and Fire");
        bbooks.insert("The Emerald City");
        bbooks.insert("The Odyssey");
        println!("{:?}", bbooks);
    }

    #[test]
    fn test_binary_heap() {
        // 优先队列：Rust提供的优先队列是基于二叉堆来实现的，
        // Rust BinaryHeap will be a max-heap. if you want to have min-heap, change the Ord trait for T
        // 二叉堆是完全二叉树或者是近似完全二叉树。二叉堆满足堆特性：
        // 父节点的键值总是保持固定的序关系于任何一个子节点的键值，且每个节点的左子树和右子树都是一个二叉堆
        let mut heap = BinaryHeap::new();
        assert_eq!(heap.peek(), None);
        let arr = [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45];
        for &i in arr.iter() {
            heap.push(i);
        }
        assert_eq!(heap.peek(), Some(&93));
        // [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45]
        println!("{:?}", heap);

        while let Some(i) = heap.pop() {
            print!("{} ", i);
        }
        // 93 80 72 53 48 45 36 35 30 18 15
        println!();
    }
}
