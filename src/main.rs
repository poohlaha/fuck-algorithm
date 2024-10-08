mod array;
mod backtrace;
mod bfs;
mod error;
mod link;
mod math;
mod string;
mod tree;

fn main() {
    link::test::test();
    array::test::test();
    string::test::test();
    tree::test::test();
    math::test::test();
    backtrace::test::test();
    bfs::test::test();
}
