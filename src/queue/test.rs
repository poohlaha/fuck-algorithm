use crate::queue::circular_array::CircularArray;
use crate::queue::circular_buffer::CircularBuffer;
use crate::queue::hash_table::{HashMap, HashTable};

fn test_circular_buffer() {
    println!("----- circular buffer start ------");
    let mut circular_buffer = CircularBuffer::new(3);
    println!("push:");
    circular_buffer.push(1);
    circular_buffer.push(2);
    circular_buffer.push(3);
    circular_buffer.print();

    println!("pop:");
    circular_buffer.pop();
    circular_buffer.pop();

    circular_buffer.push(4);
    circular_buffer.push(5);
    circular_buffer.push(6);
    circular_buffer.print();

    println!("----- circular buffer end ------");
}

fn test_circular_array() {
    println!("----- circular array start ------");
    let mut circular_array = CircularArray::new(3);

    println!("push front:");
    circular_array.push_front(1);
    circular_array.push_front(2);
    circular_array.push_front(3);
    circular_array.push_front(4);
    circular_array.print();

    println!("push back:");
    circular_array.push_back(5);
    circular_array.push_back(6);
    circular_array.push_back(7);
    circular_array.push_back(8);
    circular_array.print();

    println!("pop front:");
    circular_array.pop_front();
    circular_array.pop_front();
    circular_array.pop_front();
    circular_array.pop_front();
    circular_array.print();

    println!("pop back:");
    circular_array.pop_back();
    circular_array.pop_back();
    circular_array.pop_back();
    circular_array.pop_back();
    circular_array.print();
    println!("----- circular array end ------");
}

fn test_hash_map() {
    println!("----- hash map start ------");
    let mut hash_map = HashMap::new(3);

    println!("put:");
    hash_map.put(1, 1);
    hash_map.put(2, 2);
    hash_map.put(3, 3);
    hash_map.put(7, 7);
    hash_map.put(8, 8);
    hash_map.put(9, 9);
    hash_map.put(10, 10);
    hash_map.put(11, 11);
    hash_map.put(4, 4);
    hash_map.put(5, 5);
    hash_map.put(6, 6);
    hash_map.print();

    println!("get:");
    println!("{:?}", hash_map.get(&1));
    println!("{:?}", hash_map.get(&2));
    println!("{:?}", hash_map.get(&3));
    println!("{:?}", hash_map.get(&4));
    println!("{:?}", hash_map.get(&5));
    println!("{:?}", hash_map.get(&6));
    println!("{:?}", hash_map.get(&7));
    println!("{:?}", hash_map.get(&8));
    println!("{:?}", hash_map.get(&9));
    println!("{:?}", hash_map.get(&10));
    println!("{:?}", hash_map.get(&11));

    println!("remove:");
    hash_map.remove(&1);
    hash_map.remove(&2);
    hash_map.remove(&3);
    hash_map.remove(&4);
    hash_map.remove(&5);
    hash_map.remove(&6);
    hash_map.remove(&7);
    hash_map.remove(&8);
    hash_map.remove(&9);
    hash_map.remove(&10);
    hash_map.remove(&11);
    hash_map.print();
    println!("----- hash map end ------");
}

fn test_hash_table() {
    println!("----- hash table start ------");
    let mut hash_table = HashTable::new(3);

    fn print(bucket: Option<&std::collections::LinkedList<(i32, i32)>>) {
        if let Some(bucket) = bucket {
            print!("[");
            for (key, value) in bucket {
                print!("({}, {:?}), ", key, value);
            }
            print!("]");
            println!();
            return;
        }

        print!("(None)");
        println!();
    }

    println!("put:");
    hash_table.put(1, 1);
    hash_table.put(1, 2);
    hash_table.put(1, 3);
    hash_table.put(2, 1);
    hash_table.put(2, 2);
    hash_table.put(7, 7);
    hash_table.put(8, 8);
    hash_table.put(9, 9);
    hash_table.put(10, 10);
    hash_table.put(11, 11);
    hash_table.put(3, 3);
    hash_table.put(4, 4);
    hash_table.put(5, 5);
    hash_table.put(6, 6);
    hash_table.put(6, 7);
    hash_table.print();

    println!("get:");
    print(hash_table.get(&1));
    print(hash_table.get(&2));
    print(hash_table.get(&3));
    print(hash_table.get(&4));
    print(hash_table.get(&5));
    print(hash_table.get(&6));
    print(hash_table.get(&7));
    print(hash_table.get(&8));
    print(hash_table.get(&9));
    print(hash_table.get(&10));
    print(hash_table.get(&11));

    println!("remove:");
    hash_table.remove(&1);
    hash_table.remove(&2);
    hash_table.remove(&3);
    hash_table.remove(&4);
    hash_table.remove(&5);
    hash_table.remove(&6);
    hash_table.remove(&7);
    hash_table.remove(&8);
    hash_table.remove(&9);
    hash_table.remove(&10);
    hash_table.remove(&11);
    hash_table.print();
    println!("----- hash table end ------");
}

pub(crate) fn test() {
    println!("----- queue start ------");
    // test_circular_buffer();
    // test_circular_array();
    // test_hash_map();
    test_hash_table();
    println!("----- queue end ------");
}
