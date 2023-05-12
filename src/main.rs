//idea.... I can create my RustDB
//then host it through my own website
//then test how secure it is to injection
//based on SQLMap attacking myself...


//functionalities to implement:
//
//add functionality to match order of cols given by user
//note however this only really matters when they print
//to the console so basically when they first create
//or edit columns I'll need book keeping info to track
//ordering
//
//Load tables from files
//
//Check for valid ints when 
//inserting into int column
//
//Check for valid ints when 
//retrieving from int column
//
//Save tables to files
//
//Commands similar to SQL
//that is make a language
//
//multi threading to maybe implement
//some sort of network connectivity
//(need to implement some sort of one 
//write many readers)
//
//caching for db tables to avoid reaching out to disk
//
//
//this will be great to have as a project if I document
//it well
use std::collections::HashMap;
use std::collections::BTreeMap;


pub mod cmd_logic;
use cmd_logic::*;

pub mod db_structures;
use db_structures::*;



fn main() {
    
    let mut input = String::new();
    input = get_user_input(input);

    let input_split = input.split_whitespace();

    for word in input_split {
        println!("word = {}", word);
    }

    let my_interpreter = CommandInterpreter::new(input.clone());

    my_interpreter.interpret_command();
  
}
    
pub fn get_user_input(mut input: String) -> String {
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}












