pub fn test_my_vec_int() {
    let mut my_vec: Vec<i32> = Vec::new();
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    my_vec.push(4);
    my_vec.push(5);

    println!(
        "Length: {:?}, Capacity: {:?}",
        my_vec.len(),
        my_vec.capacity()
    );
    print!("{:?}", my_vec);
}
