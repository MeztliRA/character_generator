use std::io;
use rand::Rng;

fn main() {
    let title_picker = rand::thread_rng().gen_range(1, 5);
    
    let title = 
        if title_picker == 1 {
            String::from("brave")
        } else if title_picker == 2 {
            String::from("coward")
        } else if title_picker == 3 {
            String::from("humble")
        } else {
            String::from("wise")
        };
    
    println!("CharacterGenerator\n");

    let mut name = String::new();
    println!("Enter a name: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = remove_newline(&mut name);
        
    let mut town = String::new();
    println!("\nEnter a town: ");

    io::stdin()
        .read_line(&mut town)
        .expect("Failed to read line");

    let town = remove_newline(&mut town);

    let mut age = String::new();
    println!("\nEnter a number: ");

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    
    let age: i32 = match age.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };
    
    let mut pet = String::new();
    println!("\nEnter a animal: ");
    
    io::stdin()
        .read_line(&mut pet)
        .expect("Failed to read line");

    let pet = remove_newline(&mut pet);    

    generate_character(&name, &town, &age, &pet, &title);
}

fn generate_character(iname: &String, itown: &String, iage: &i32, ipet: &String, ititle: &String) {
    println!("\n    _    ");
    println!("   |_|     name: {} the {}", iname, ititle);
    println!("    |      town of origin: {}", itown);
    println!("   /|\\     age: {} years old", iage);
    println!("    |      pet: {}", ipet);
    println!("   / \\    ");
}

fn remove_newline(s: &mut String) -> String {
    let s = s.replace("\n", "");
    return s;
}