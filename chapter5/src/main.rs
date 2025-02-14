use std::{collections::HashMap, io::{self, BufRead}};

fn main() {
    let mut context: HashMap<String,String> = HashMap::new();
    context.insert("name".to_string(), "Bob".to_string());
    context.insert("city".to_string(), "Boston".to_string());

    for line in io::stdin().lock().lines(){
        
    }


}
