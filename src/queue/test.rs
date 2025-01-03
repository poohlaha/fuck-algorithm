use crate::queue::circular_array::CircularArray;
use crate::queue::circular_buffer::CircularBuffer;

pub(crate) fn test_circular_buffer() {
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

pub(crate) fn test_circular_array() {
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

pub(crate) fn test() {
    println!("----- queue start ------");
    test_circular_buffer();
    test_circular_array();
    println!("----- queue end ------");
}
