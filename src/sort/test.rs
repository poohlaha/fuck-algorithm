use crate::sort::{basic, distributed, highly, special};

pub fn test() {
    println!("----- sort start ------");
    basic::test::test();
    distributed::test::test();
    highly::test::test();
    special::test::test();
    println!("----- sort end ------");
}
