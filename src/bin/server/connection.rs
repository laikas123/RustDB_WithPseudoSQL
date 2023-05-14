/// Handle a single client's connection.

use async_rusql::{FromClient, FromServer};

//note this line is the same as these two:
//use async_Rusql::utils;
//use async_Rusql::utils::RusqlResult;
use async_rusql::utils::{self, RusqlResult};
use async_std::prelude::*;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::sync::Arc;

use crate::group_table::GroupTable;

pub async fn serve(socket: TcpStream, groups: Arc<GroupTable>)
                   -> RusqlResult<()>
{
    //ownership of Outbound is given to the Arc,
    //so this is an atomic reference count pointer 
    //to an Outbound on the heap
    //note that the socket is cloned... so no move occurs
    let outbound = Arc::new(Outbound::new(socket.clone()));

    //create a new buffered read of the socket
    let buffered = BufReader::new(socket);

    //returns stream of Rusqlresult
    let mut from_client = utils::receive_as_json(buffered);

    //iterate over the request result (type RusqlResult)
    while let Some(request_result) = from_client.next().await {
        
        //unwrap the result
        let request = request_result?;

        //figure out if we are dealing with a join packet
        //or a post packet
        let result = match request {
            //note FromClient is an enum,
            //and so here we are matching an
            //enum with data and then accessing that
            //data
            FromClient::Join { group_name } => {
                let group = groups.get_or_create(group_name);
                group.join(outbound.clone());
                Ok(())
            }

            FromClient::Query { command } => {
                println!("Received a command: {}", command);
                Ok(())
            }

            FromClient::Post { group_name, message } => {
                match groups.get(&group_name) {
                    //note this is really Arc<Group> but what's neat is
                    //it just behaves normally when you call things
                    //like its methods
                    Some(group) => {
                        group.post(message);
                        Ok(())
                    }
                    None => {
                        Err(format!("Group '{}' does not exist", group_name))
                    }
                }
            }
        };

        if let Err(message) = result {
            let report = FromServer::Error(message);
            outbound.send(report).await?;
        }
    }

    Ok(())
}


//note we are using async Mutex so it
//needs to be awaited
use async_std::sync::Mutex;


//ok so we have a struct "Outbound"
//and note this is what's called a 
//tuple struct
//
//From the docs:
//"Tuple structs are useful when you 
//want to give the whole tuple a name 
//and make the tuple a different type 
//from other tuples, and when naming 
//each field as in a regular struct would 
//be verbose or redundant."
//
//"tuple struct instances are similar to tuples in 
//that you can destructure them into their individual 
//pieces, and you can use a . followed by the index 
//to access an individual value."
//
//so long story short Outbound is a Mutex
//that protects a TcpStream
pub struct Outbound(Mutex<TcpStream>);

impl Outbound {

    //so the constructor is given a TcpStream
    //for a client and a Mutex for this TcpStream
    //is created
    pub fn new(to_client: TcpStream) -> Outbound {
        Outbound(Mutex::new(to_client))
    }

    //an async function, so it returns a future of RusqlResult
    pub async fn send(&self, packet: FromServer) -> RusqlResult<()> {
        //call the lock on the TcpStream
        let mut guard = self.0.lock().await;
        //so from the utils file, we call send_as_json
        //so presumably send the packet to the client
        //note that we give a writable reference to the
        //mutex derefed so basically the tcp stream
        utils::send_as_json(&mut *guard, &packet).await?;
        guard.flush().await?;
        Ok(())
    }
}
