use crate::data_structure::{array, hash, link, queue};

pub fn test() {
    println!("----- data structure start ------");
    array::test::test();
    link::test::test();
    queue::test::test();
    hash::test::test();
    println!("----- data structure end ------");
}
