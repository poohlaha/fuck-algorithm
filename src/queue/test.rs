use crate::queue::circular::CircularArray;

pub(crate) fn test_circular_array() {
    println!("----- circular start ------");
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
    println!("----- circular end ------");
}

pub(crate) fn test() {
    println!("----- queue start ------");
    test_circular_array();
    println!("----- queue end ------");
}
