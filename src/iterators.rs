use std::slice::Iter;

pub fn test_rust_iterators() {
    let mut my_vec: Vec<&str> = vec!["ali", "sule", "downplay", "disguise"];

    let mut my_vec_iter: Iter<'_, &str> = my_vec.iter();

    let item01: Option<&&str> = my_vec_iter.next();

    let mut fruits: Vec<&str> = vec!["apple", "banana", "cherry"];
    let mut vegetables: Vec<&str> = vec!["carrot", "beetroot", "asparagus"];

    let aggregated_iter = fruits.iter().chain(vegetables.iter());

    let all_gredients: Vec<&&str> = aggregated_iter.collect();

    for item in &all_gredients {
        println!("{}", item);
    }
    println!("{:?}", all_gredients);

    let string_gredients = fruits.iter().map(|x| String::from(*x));
    let new_ingredients = string_gredients.map(|mut e: String| {
        e.push_str(" new");
        return e;
    });
    new_ingredients.for_each(|x| println!("{}", x));
}
