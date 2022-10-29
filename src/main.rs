use inquire::{Select, Text};
extern crate rpassword;
use rpassword::read_password;
use std::io::Write;
use shop::crates::{self, Item};
fn main() {

    'logger: loop {
        println!("WELCOME TO PRAISE STORE \nPlease input admin password");
        println!("default password is praise027");
        std::io::stdout().flush().unwrap();
        let password = read_password().unwrap();
       if crates::valid(password) == true{
        break 'logger
       }
       else {
        println! { "incorrect password!"};
        continue;
       }
    }

    println!("WELCOME TO PRAISE STORE");
    let mut database = crates::load();
    'menu:loop{
        let options:Vec<_> = vec!["Purchase Item", "View Item", "View all Items", "Add New Item", "Change Price", "Update Item Quantity","Exit"];
            let ans = Select::new("MENU", options).prompt();
             match ans {
            Ok(choice) => if choice == "Purchase Item"{
                let mut name = String::new();
                let _name = Text::new("Input item name...").prompt();
                match _name {
                    Ok(_name) => name.push_str(_name.as_str()),
                    Err(_) => println!("An error happened when parsing the name, try again later."),
                }
                let database_=database.clone();
                let _item = crates::find(name, database_);
                let item = match _item {
                    Some(_item) => _item,
                    None => continue,
                };
                for data in database.iter_mut() {
                    if *data.name == item[0].name {
                        let invoice = Item::purchase(data);
                        match invoice {
                            Some(invoice) =>
                            println!("Item {}\tQuantity{}\tTotal amount{}", invoice.1, invoice.2, invoice.0),
                            None => continue,
                        }
                    }
                }
                crates::save(&database);

            }

            else if choice == "View all Items" {
                for item in database.iter() {
                    println!("{:?}", item);
                }
            }

            else if choice == "View Item" {
                let mut name = String::new();
                let _name = Text::new("Input item name...").prompt();
                match _name {
                    Ok(_name) => name.push_str(_name.as_str()),
                    Err(_) => println!("An error happened when parsing the name, try again later."),
                }
                let database_= database.clone();
                let item = crates::find(name, database_);
                match item { 
                    Some(item) => println!("{:?}", item),
                    None => continue,
                }

            } 
            else if choice == "Add New Item" { // creates new item and save
               let new_item = Item::new_item();
               database.push(new_item);
               crates::save(&database);

            }
            else if choice == "Change Price" {
                let mut name = String::new();
                let _name = Text::new("Input item name...").prompt();
                match _name {
                    Ok(_name) => name.push_str(_name.as_str()),
                    Err(_) => println!("An error happened when parsing the name, try again later."),
                }
                let database_= database.clone();
                let _item = crates::find(name, database_);
                let item = match _item {
                    Some(_item) => _item,
                    None => continue,
                };

                for data in database.iter_mut() {
                    if *data.name == item[0].name {
                        Item::change_price(data);
                    }
                }
                crates::save(&database);
            }
            else if choice ==  "Update Item Quantity" {
                let mut name = String::new();
                let _name = Text::new("Input item name...").prompt();
                match _name {
                    Ok(_name) => name.push_str(_name.as_str()),
                    Err(_) => println!("An error happened when parsing the name, try again later."),
                }
                let database_= database.clone();
                let _item = crates::find(name, database_);
                let item = match _item {
                    Some(_item) => _item,
                    None => continue,
                };

                for data in database.iter_mut() {
                    if *data.name == item[0].name {
                        Item::update_stock(data);
                    }
                }
                crates::save(&database);
            } 
            else {
                println!("Exiting");
                break 'menu;
            },
            Err(_) => println!("There was an error with your selection, please try again"),
        }
    }
}
        
