pub fn generate_character(name: &String, town: &String, age: &i32, pet: &String, title: &String, pet_name: &String) {
    println!("\n    _    ");
    println!("   |_|     name: {} the {}", name, title);
    println!("    |      town of origin: {}", town);
    println!("   /|\\     age: {} years old", age);
    println!("    |      pet: {} the {}", pet_name, pet);
    println!("   / \\    ");
}