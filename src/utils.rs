//! Utilities for both clients and servers.

use std::error::Error;

//this is just for custom error type
pub type RusqlError = Box<dyn Error + Send + Sync + 'static>;
pub type RusqlResult<T> = Result<T, RusqlError>;

use async_std::prelude::*;
use serde::Serialize;
use std::marker::Unpin;

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
