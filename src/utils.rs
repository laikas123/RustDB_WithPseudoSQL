//! Utilities for both clients and servers.

use std::error::Error;

//this is just for custom error type
pub type RusqlError = Box<dyn Error + Send + Sync + 'static>;
pub type RusqlResult<T> = Result<T, RusqlError>;

use async_std::prelude::*;
use serde::Serialize;
use std::marker::Unpin;

static DB_DIR: &str = "/home/logan/database/";

pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> RusqlResult<()>
where
    S: async_std::io::Write + Unpin,
    P: Serialize,
{
    //packet gets turned into json string
    let mut json = serde_json::to_string(&packet)?;
    //add a new line to the end
    json.push('\n');
    //write to the tcp stream, return error if occurs
    outbound.write_all(json.as_bytes()).await?;
    //return ok val
    Ok(())
}

use serde::de::DeserializeOwned;
// use serde::de::Deserialize;

//so this function is basically transforming each line of the 
//input via the map adapter so that we always get a proper string
//this is also what is returned, note that we are are returning something
//that implements stream with type Rusql result, and note that stream
//is the async equivalent of iterator
pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = RusqlResult<P>>
    where S: async_std::io::BufRead + Unpin,
          P: DeserializeOwned,
{
    inbound.lines()
        .map(|line_result| -> RusqlResult<P> {
            let line = line_result?;
            let parsed = serde_json::from_str::<P>(&line)?;
            Ok(parsed)
        })
}

//another cool functionality to add would be to export 
//specific parts of the database to a file, e.g. a specific 
//table 
//
//cause then you could easily add tables from one db to another
//using .table file maybe...


use std::io::Write;

//note I didn't need to even tell it what my database 
//structure was thanks to the function just knowing
//it can be serialized, that's all it cared about...
pub fn db_to_json_file<P>(db: P, db_name: &str) -> RusqlResult<()> 
    where P: Serialize
{

    let mut f = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(DB_DIR.to_owned()+db_name+".db")?;
    let json_string = serde_json::to_string(&db)?;
    f.write_all(json_string.as_bytes())?;
    f.flush()?;

    Ok(())
}
