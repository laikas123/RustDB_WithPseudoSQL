use std::collections::HashMap;

//import siblings 
use super::db_structures::*;
use super::cmd_logic::*;
use super::cmd_interpreter::*;
use async_rusql::utils::RusqlResult;
use async_rusql::utils::{self};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Permission {
    Read,
    ReadWrite,
}

#[derive(Debug)]
pub struct Connection {
    pub permission: Permission,
    pub command: String,
    //this is really a pseudo field for now
    //in reality this will be whenever the connection
    //sends in new data from its "stream"
    pub can_execute: bool,
}


#[derive(Debug)]
pub enum StatusFlag {
    Dirty,
    Clean,
}

#[derive(Debug)]
// need a field or method to create an actual listener for tcp connections...
pub struct LoadBalancer {
    //each connection will have id for fast lookup
    pub connections: HashMap<String, Connection>,
    //this is just each database in memory
    //key is database name, val is Db and status
    pub pages: HashMap<String, (StatusFlag, Db)>,
    pub cmd_interpreter: CmdInterpreter,
}


impl LoadBalancer {

    pub fn new() -> Self {
        LoadBalancer{
            connections: HashMap::new(), 
            pages: HashMap::new(),
            cmd_interpreter: CmdInterpreter::new(),
        }
    }

    pub fn execute_cmd(&mut self){

        self.cmd_interpreter.set_btreemap("create_db horses".to_string());
        self.cmd_interpreter.interpret_command(&mut self.pages, "".to_string());

        self.pretty_print();

        self.cmd_interpreter.set_btreemap("create_table horseshoes type RS quality RS quantity RI price RI".to_string());
        self.cmd_interpreter.interpret_command(&mut self.pages, "horses".to_string());

        self.pretty_print();

        self.cmd_interpreter.set_btreemap("insert_into horseshoes metal good 33 140".to_string());
        self.cmd_interpreter.interpret_command(&mut self.pages, "horses".to_string());

        self.pretty_print();

    }


    pub fn pretty_print(&self) {
        for (_, db_tuple) in &self.pages{
            db_tuple.1.pretty_print_tables();
        }
    }

    pub fn db_to_file(&self) -> RusqlResult<()>{
        match self.pages.get("horses"){
            Some(db_tuple) => {
                utils::db_to_json_file(&db_tuple.1, "horses");
                return Ok(());
            },
            _ => {
                return Err("Could not write db to file...".into());
            },
        }
        
    }



    // pub fn handle_connections(&self) {

    //     loop {
    //         for connection in &self.connections{
                
    //             //equivalent of when the client has sent data
    //             //over their connection
    //             if connection.can_execute{
                    
    //             }

    //         }
    //     }


    // }


}
