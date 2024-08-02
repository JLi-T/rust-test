//format! is important 
use std::io;
use std::thread;
use std::time::Duration;


fn add_todo(inv: &mut Vec<String>, items: &str) {
    let mut count = 1;
    for item in items.split_whitespace() {
        let item = item.to_string();
        if !inv.contains(&item){
            inv.push(format!("{} - {}", count, item));
            count +=1;
        }
    }
    println!("added {}", items);
    thread::sleep(Duration::from_secs(1));
}

fn remove_todo(inv: &mut Vec<String>, indexes: &[usize]) {
    // makes copy of indexes
    let mut indexes = indexes.to_vec();
    //usually a.cmp(b) but we are removing from the end first because of shifts that would occur from removing from beginning
    indexes.sort_unstable_by(|a,b| b.cmp(a));
    for &index in &indexes {
        if index < inv.len() {
            if let Some(item) = inv.get_mut(index) {
                *item = format!("\x1b[31m{}\x1b[0m", item);
            }
        }
    }
    thread::sleep(Duration::from_secs(1));
}

fn show_todo(inv: &[String]) {
    // let mut count = 0;
    println!("\nYOUR TODO LIST");
    for item in inv{
        println!("\n{item}");
    }
    thread::sleep(Duration::from_secs(1));
}

fn main(){
    println!("Welcome to your todo list!");
    let mut inv = Vec::new();
    // let test = vec!["banana".to_string(), "apple".to_string(), "mango".to_string()];
    // for item in test {
    //     add_todo(&mut inv, &item);
    // }
    // show_todo(&inv);

    loop{
        println!("\n1. add items to todo list by writing ----- 'todo add' followed by items");
        println!("2. remove items from todo list by writing ----- 'todo remove' followed by item indexes");
        println!("3. to view todo list write ----- 'todo'");
        println!("4. to exit write ----- 'todo exit'\n");

        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt).expect("failed to read");
        let prompt = prompt.trim();
    
        match prompt{
            "todo" => {show_todo(&inv)}
            "todo exit" => {
                println!("exiting program");
                break;
            }
            _ => {
                if prompt.starts_with("todo add ") {
                    let items = prompt.replace("todo add ", "");
                    add_todo(&mut inv, &items);
                } else if prompt.starts_with("todo remove ") {
                    let indexes_str = prompt.replace("todo remove ", "");
                    let indexes: Vec<usize> = indexes_str
                    .split_whitespace()
                    //minus 1 because index 0 and user probably doesn't know that
                    .filter_map(|s| s.parse::<usize>().ok().map(|num| num-1))
                    .collect();
                remove_todo(&mut inv, &indexes);
                } else{
                    println!("invalid command!")
                }
            }

        }
    }
}