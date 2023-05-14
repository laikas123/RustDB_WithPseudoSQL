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
//
//
//I will probably need to make operations such as
//editing the database atomic especially since the rows
//are separated by types so what if the system crashes after
//updating a string row but not the int row....
//
//
//todo give better helfpul error messages
//when commands fail
use std::collections::HashMap;
use std::collections::BTreeMap;


pub mod cmd_logic;
use cmd_logic::*;

pub mod db_structures;
use db_structures::*;

pub mod load_balancer;
use load_balancer::*;

pub mod cmd_interpreter;
use cmd_interpreter::*;



fn main() {
    
    // let mut input = String::new();
    // // input = get_user_input(input);

    // input = "create_db horses".to_string();
    // let mut my_interpreter = CommandInterpreter::new(input.clone());
    // my_interpreter.interpret_command();
    // my_interpreter.pretty_print();

    // input = "create_table horseshoes type RS quality RS quantity RI price RI".to_string();
    // my_interpreter.set_btreemap(input.clone());
    // my_interpreter.interpret_command();
    // my_interpreter.pretty_print();

    // input = "insert_into horseshoes metal good 33 140".to_string();
    // my_interpreter.set_btreemap(input.clone());
    // my_interpreter.interpret_command();
    // my_interpreter.pretty_print();

    // loop{
    //     input = get_user_input(input);
    //     my_interpreter.set_btreemap(input.clone());
    //     my_interpreter.interpret_command();
    //     my_interpreter.pretty_print();
    // }


    let mut my_lb = LoadBalancer::new();

    my_lb.execute_cmd();

}
    
pub fn get_user_input(mut input: String) -> String {
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}












