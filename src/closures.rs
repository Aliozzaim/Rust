struct Person {
    name: String,
    last_name: String,
}
pub fn test_closures() {
    let mut person1: Person = Person {
        name: "deli".to_string(),
        last_name: "veli".to_string(),
    };
    let mut change_the_person_name = || person1.last_name = "ALI".to_string();
    change_the_person_name();
    println!("{0} {1}",person1.last_name ,person1.name)
}
