use std::{
    collections::{HashMap, HashSet},
    hash,
};
pub fn test_my_hashmap() {
    let mut stock_list: HashMap<String, f32> = HashMap::new();

    stock_list.insert("AAPL".to_string(), 123.45);
    stock_list.insert("GOOGL".to_string(), 543.21);
    stock_list.insert("NVDA".to_string(), 987.65);

    println!("{:?}", stock_list);
    print!("{:?}", stock_list.len());

    for (key, value) in &stock_list {
        println!("key {}:  value :{}", key, value);
    }

    let mut stock_list2 = HashSet::from([1, 3, 2]);
    for i in stock_list2.drain() {
        print!(" {i}");
    }
    print!(" {:?}", stock_list2);
}
