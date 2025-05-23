use crate::data_structure::{array, hash, link, queue, tree};

pub fn test() {
    println!("----- data structure start ------");
    array::test::test();
    link::test::test();
    queue::test::test();
    hash::test::test();
    tree::test::test();
    println!("----- data structure end ------");
}
