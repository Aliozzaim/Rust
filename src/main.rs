pub mod helpers;

#[allow(dead_code)]
fn main() {
    let myresult: String = helpers::get_full_name("Ali", "V");
    println!("Hello, world! {}", myresult);
    test_if();
}
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
