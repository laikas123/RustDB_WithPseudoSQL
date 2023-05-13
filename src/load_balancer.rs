

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Permission {
    Read,
    ReadWrite,
}


pub struct Connection {
    pub permission: Permission,
    pub command: String,
    //this is really a pseudo field for now
    //
    pub CanExecute: bool,
}


pub struct BufferedReaders {

}

pub struct BufferedReadWriters {

}