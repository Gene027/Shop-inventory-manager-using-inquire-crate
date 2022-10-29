pub mod crates { 
    use std::fs::{OpenOptions, File};
    use std::io::{Write, Read};
    use serde::{Deserialize, Serialize};
    use inquire::Text;
    
    #[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Item {
    pub name: String,
    pub item_type: String,
    pub price: f64,
    pub stock_qty: u64,
}

 //associated functions, create new instance or modify
impl Item {
    pub fn new_item() -> Item {
        let mut name = String::new();
        let mut item_type = String::new();
        let mut price: Vec<f64> = vec![];
        let mut stock_qty: Vec<u64> = vec![];  
        let _name = Text::new("Input new item name...").prompt();
                match _name {
                    Ok(_name) => name.push_str(_name.to_lowercase().as_str()),
                    Err(_) => println!("An error happened when parsing the name, try again later."),
                }
        
        let _item_type = Text::new("Input the item type...").prompt();
                match _item_type {
                    Ok(_item_type) => item_type.push_str(_item_type.as_str()),
                    Err(_) => println!("An error happened when inputing the item type, try again later."),
                }
           
        let string_price = Text::new("Input new item price...").prompt();
                match string_price {
                    Ok(string_price) => price.push(string_price.parse().expect("could not parse price")),
                    Err(_) => println!("An error happened when parsing the price, try again later."),
                }                
        let string_stock_qty = Text::new("Input new item quantity...").prompt();
                match string_stock_qty {
                    Ok(string_stock_qty) => stock_qty.push(string_stock_qty.parse().expect("could not parse quantity")),
                    Err(_) => println!("An error happened when parsing the quantity, try again later."),
                }        

        Item { name,
               item_type,
               price : price[0],
               stock_qty : stock_qty [0]
         }
    }
}

//methods, use values in Items
impl Item {
    pub fn update_stock (&mut self) {
        //print the details of self item to edit
        let mut amount: u64 = 0;
        let amount_buff =  Text::new("quantity of new stock").prompt();
        match amount_buff {
            Ok(amount_buff) => amount += amount_buff.parse::<u64>().expect("could not parse price"),
            Err(_) => println!("An error happened when parsing the price, try again later."),
        }
        self.stock_qty += amount;
    }

    pub fn change_price (&mut self) {
        println!("Old price {}", &self.price);
        let mut amount: f64 = 0.0;
        let amount_buff =  Text::new("new price").prompt();
        match amount_buff {
            Ok(amount_buff) => amount += amount_buff.parse::<f64>().expect("could not parse price"),
            Err(_) => println!("An error happened when parsing the price, try again later."),
        }
        self.price = amount;
    }

     pub fn purchase<'a> (&'a mut self) -> Option<(f64, &String, u64)> {
        let mut qty: u64 = 0;
        let amount_buff =  Text::new("input quantity to buy").prompt();
        match amount_buff {
            Ok(amount_buff) => qty += amount_buff.parse::<u64>().expect("could not parse quantity"),
            Err(_) => println!("An error happened when parsing the quantity, try again later."),
        }
            if self.stock_qty >= qty {
                self.stock_qty -= qty;
                //print qty and return price
                let purchased = (self.price * qty as f64, &self.name, qty);
                return Some(purchased)
            }
            else {
                println!("insufficient stock");
                None
            }
        }
        
}


pub fn find (name: String, database_:Vec<Item>) -> Option<Vec<Item>> {

    let mut result: Vec<Item> = Vec::new();
    let name = name.to_lowercase();
   for item in database_.iter() {
    if *item.name == name {
        result.push(item.clone())
    }
}

    if result.is_empty() {
        println!("Item not found");
        return None;
    }
    return Some(result);
   }
   


pub fn valid (password: String) -> bool {
    password == "praise027"
}

pub fn save (database:&Vec<Item>) {
    let mut f =OpenOptions::new()
    .write(true).read(true).open("database.bin").unwrap();
    let mut buffer = String::new();
    for item in database.iter() {
        let serialized = serde_json::to_string(&item).unwrap();
        buffer = format!("{}\n{}", buffer, serialized);
    }
    f.write_all(buffer.as_bytes()).expect("writing to database failed");
    println!("State has been saved");
}

pub fn load () -> Vec<Item> { 
    let mut database: String = String::new();
    let mut f = File::open("database.bin").unwrap();
    f.read_to_string(&mut database).expect("unable to read database.bin to database");
    
    let mut items: Vec<Item> = vec![];
    for line in database.lines() {
        if line.is_empty() {
            continue;
        }
        else { 
            let deserialized: Item = serde_json::from_str(&line).unwrap();
        items.push(deserialized);
        }
    }
    items
}
}