use std::io;
use rand::Rng;
use anyhow::{Context, Result};

fn main() -> Result<()> {
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

    io::stdin().read_line(&mut name)
        .with_context(|| format!("Failed to read line."))?;

    let name = remove_newline(&mut name);
        
    let mut town = String::new();
    println!("\nEnter a town: ");

    io::stdin().read_line(&mut town)
        .with_context(|| format!("Failed to read line."))?;

    let town = remove_newline(&mut town);

    let mut age = String::new();
    println!("\nEnter a number: ");

    io::stdin().read_line(&mut age)
        .with_context(|| format!("Failed to read line."))?;
    
    let age: i32 = age.trim().parse()
        .with_context(|| format!("Failed to convert to integer"))?;
    
    let mut pet = String::new();
    println!("\nEnter a animal: ");
    
    io::stdin().read_line(&mut pet)
        .with_context(|| format!("Failed to read line."))?;
    
    let pet = remove_newline(&mut pet);

    let mut pet_name = String::new();
    println!("\nEnter a name: ");

    io::stdin().read_line(&mut pet_name)
        .with_context(|| format!("Failed to read line."))?;

    let pet_name = remove_newline(&mut pet_name);

    generate_character(&name, &town, &age, &pet, &title, &pet_name);

    Ok(())
}

fn generate_character(iname: &String, itown: &String, iage: &i32, ipet: &String, ititle: &String, ipet_name: &String) {
    println!("\n    _    ");
    println!("   |_|     name: {} the {}", iname, ititle);
    println!("    |      town of origin: {}", itown);
    println!("   /|\\     age: {} years old", iage);
    println!("    |      pet: {} the {}", ipet_name, ipet);
    println!("   / \\    ");
}

fn remove_newline(s: &mut String) -> String {
    let s = s.replace("\n", "");
    return s;
}