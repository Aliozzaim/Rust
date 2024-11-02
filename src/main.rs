pub mod closures;
pub mod helpers;
pub mod iterators;
pub mod my_hashmap;
pub mod mystructs;
pub mod vec;

#[allow(dead_code)]
fn main() {
    // test_while()
    // test_if();
    // test_for_array();
    // closures::test_closures();
    // mystructs::new_person("Ali".to_string(), "V".to_string(), 1990, 12);
    // mystructs::test_mystructs();
    // vec::test_my_vec_int();
    // my_hashmap::test_my_hashmap();
    iterators::test_rust_iterators();
}

#[allow(dead_code)]
fn test_for_array() {
    let age_array: [i32; 17] = [
        123, 23, 325, 546, 854, 23, 34, 452, 43, 5, 4, 7, 9, 5, 342, 54, 5432,
    ];
    for age in age_array {
        if age > 16 {
            println!("you can old enoght to drive a car because you are {0}", age)
        } else {
            println!(
                "you can not old enought to drive a car because you are {0}",
                age
            )
        }
    }
}
#[allow(dead_code)]
fn test_while() {
    let age_to_drive: u8 = 16;
    let mut current_age: u8 = 0;
    while current_age < age_to_drive {
        println!("You are {} years old", current_age);
        current_age += 1; // Increment `current_age`
    }
}
#[allow(dead_code)]
fn test_if() {
    let drive_age: u8 = 16;
    let mut myinput = String::new();
    println!("give me number");
    std::io::stdin().read_line(&mut myinput).unwrap();

    let age: u8 = myinput.trim().parse().unwrap_or(0);

    if age >= drive_age {
        println!("you can drive")
    } else if age == 15 || age > 10 {
        println!("else if ");
    } else {
        println!("else");
    }
}
