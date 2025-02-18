use crate::data_structure::hash::{chain, linear, open};

/// 测试 `链式地址法(拉链法)`
fn test_chain() {
    let mut hash_table = chain::HashTable::new(4);
    println!("chain add:");
    hash_table.insert(1, "One");
    hash_table.insert(2, "Two");
    hash_table.insert(3, "Three");
    hash_table.insert(4, "Four");
    hash_table.insert(5, "Five");
    hash_table.insert(6, "Six");
    hash_table.print();

    println!("remove:");
    hash_table.remove(1);
    hash_table.remove(3);
    hash_table.remove(6);
    hash_table.print();

    println!("search:");
    println!("search 2: {:?}", hash_table.search(2));
    println!("search 4: {:?}", hash_table.search(4));
    println!();
}

/// 测试 `开放寻址法`
fn test_open() {
    let mut hash_table = open::HashTable::new(4);
    println!("open add:");
    hash_table.insert(1, "One");
    hash_table.insert(2, "Two");
    hash_table.insert(3, "Three");
    hash_table.insert(4, "Four");
    hash_table.insert(5, "Five");
    hash_table.insert(6, "Six");
    hash_table.print();

    println!("remove:");
    hash_table.remove(1);
    hash_table.remove(3);
    hash_table.remove(6);
    hash_table.print();

    println!("search:");
    println!("search 2: {:?}", hash_table.search(2));
    println!("search 4: {:?}", hash_table.search(4));
    println!();
}

/// 测试 `开放寻址法(线性探查法)`
fn test_linear() {
    let mut hash_table = linear::HashTable::new(4);
    println!("linear add:");
    hash_table.insert(1, "One");
    hash_table.insert(2, "Two");
    hash_table.insert(3, "Three");
    hash_table.insert(4, "Four");
    hash_table.insert(5, "Five");
    hash_table.insert(6, "Six");
    hash_table.print();

    println!("remove:");
    hash_table.remove(1);
    hash_table.remove(3);
    hash_table.remove(6);
    hash_table.print();

    println!("search:");
    println!("search 2: {:?}", hash_table.search(2));
    println!("search 4: {:?}", hash_table.search(4));
    println!();
}

pub fn test() {
    println!("----- hash table start ------");
    test_chain();
    test_open();
    test_linear();
    println!("----- hash table end ------");
}
