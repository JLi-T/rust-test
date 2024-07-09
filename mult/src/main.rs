//multiply
// fn main() {
//     for i in 1..10{
//         println!("5x{} = {}", i, 5*i)
//     }
// }

//hashmap == collection
use std::collections::HashMap;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

//item_name is String in add but &str in remove because add item has to own the string

//Metheods are from Hashmap, for example the use of *
fn add_item(inv: &mut HashMap<String, i32>, item_name: &str, quantity: i32){
    let entry = inv.entry(item_name.to_string()).or_insert(0);
    *entry += quantity;
    println!("\nAdded {} {} to inventory", quantity, item_name);
}

fn remove_item(inv: &mut HashMap<String, i32>, item_name: &str, quantity: i32){
    // tries to find the entry in inv, otherwise it is not found --> else
    if let Some(entry) = inv.get_mut(item_name){
        if *entry >= quantity {
            *entry -= quantity;
            println!("\nremoved {} {} to inventory", quantity, item_name);
            if *entry == 0 {
                inv.remove(item_name);
                println!("\nremoved all of {item_name}");
            }
        } else{
            println!("\nnot enough {} in inventory", item_name);
        }
    } else{
        println!("\n{} does not exist", item_name);
    }
}

fn display_inv(inv: HashMap<String, i32>) {
    println!("Current collection: ");
    for (item, quantity) in inv {
        println!("{}: {}", item, quantity);
    }
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let mut inv: HashMap<String, i32> = HashMap::new();

    loop{
        println!("\n1. Add item to inventory");
        println!("2. Remove item from inventory");
        println!("3. Display inventory");
        println!("4. Exit");    
        print!("Enter your choice (1-4): ");

        //makes lines go into terminal right away. Rust has a buffer that stores and delays that, this bypasses it
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("\nfailed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("enter item name");
                io::stdout().flush().unwrap();
                let mut item_name = String::new();
                io::stdin().read_line(&mut item_name).expect("\nFailed to read line");
                //to string for strings
                //trim() returns a &str by default
                let item_name = item_name.trim();

                print!("enter quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).expect("\nFailed to read line");
                //parse for numbers
                let quantity: i32 = quantity.trim().parse().expect("\nPlease enter a number");

                add_item(&mut inv, item_name, quantity);
            }
            "2" => {
                println!("enter item name");
                io::stdout().flush().unwrap();
                let mut item_name = String::new();
                io::stdin().read_line(&mut item_name).expect("\nFailed to read line");
                let item_name = item_name.trim();

                print!("enter quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).expect("\nFailed to read line");
                let quantity: i32 = quantity.trim().parse().expect("\nPlease enter a number");

                remove_item(&mut inv, item_name, quantity);
            }
            "3" => {
                display_inv(inv.clone());
            }
            "4" => {
                println!("exiting program ...");
                break;
            }
            _ => {
                println!("\nInvalid Choice");
            }
        }
    }
}
