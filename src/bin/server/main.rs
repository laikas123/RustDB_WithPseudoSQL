
// use std::collections::HashMap;
// use std::collections::BTreeMap;


// pub mod cmd_logic;
// use cmd_logic::*;

// pub mod db_structures;
// use db_structures::*;

// pub mod load_balancer;
// use load_balancer::*;

// pub mod cmd_interpreter;
// use cmd_interpreter::*;



// fn main() {
   


//     let mut my_lb = LoadBalancer::new();

//     my_lb.execute_cmd();

// }
    
// pub fn get_user_input(mut input: String) -> String {
//     input.clear();
//     std::io::stdin().read_line(&mut input).unwrap();
//     input.pop();
//     input
// }



//! Asynchronous Rusql server.
#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use async_std::prelude::*;
use async_rusql::utils::RusqlResult;
use std::sync::Arc;

mod connection;
mod group;
mod group_table;

use connection::serve;

fn main() -> RusqlResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS");

    let Rusql_group_table = Arc::new(group_table::GroupTable::new());

    async_std::task::block_on(async {
        // This code was shown in the chapter introduction.
        use async_std::{net, task};

        let listener = net::TcpListener::bind(address).await?;

        let mut new_connections = listener.incoming();
        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let groups = Rusql_group_table.clone();
            task::spawn(async {
                log_error(serve(socket, groups).await);
            });
        }

        Ok(())
    })
}

fn log_error(result: RusqlResult<()>) {
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}












