use std::io;
use rand::Rng;
use anyhow::{Context, Result};

mod generate;

fn main() -> Result<()> {
    let title_picker = rand::thread_rng().gen_range(1, 6);
    
    let title = 
        if title_picker == 1 {
            String::from("brave")
        } else if title_picker == 2 {
            String::from("coward")
        } else if title_picker == 3 {
            String::from("humble")
        } else if title_picker == 4 {
            String::from("wise")
        } else {
            String::from("giant")
        };
    
    println!("CharacterGenerator\n");

    let mut name = String::new();
    println!("Enter a name: ");

    io::stdin().read_line(&mut name)
        .with_context(|| format!("Failed to read line."))?;

    let name = String::from(name.trim());
        
    let mut town = String::new();
    println!("\nEnter a town: ");

    io::stdin().read_line(&mut town)
        .with_context(|| format!("Failed to read line."))?;

    let town = String::from(town.trim());

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
    
    let pet = String::from(pet.trim());

    let mut pet_name = String::new();
    println!("\nEnter a name: ");

    io::stdin().read_line(&mut pet_name)
        .with_context(|| format!("Failed to read line."))?;

    let pet_name = String::from(pet_name.trim());

    generate::generate_character(&name, &town, &age, &pet, &title, &pet_name);

    Ok(())
}