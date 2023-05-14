use crate::group::Group;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GroupTable(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl GroupTable {
    pub fn new() -> GroupTable {
        GroupTable(Mutex::new(HashMap::new()))
    }


    //for posting to group since group better exist
    pub fn get(&self, name: &String) -> Option<Arc<Group>> {
        self.0.lock()  //lock acquires the mutex
            .unwrap()  //unwrap avoids errrors
            .get(name) //get refers to the hashmap of the mutex, so if it succeeds it will get a Arc of Group
            .cloned()  //gets clone so that no move occurs it seems?
    }


    //for joining group since it makes sense to create if 
    //it doesn't exist yet..
    pub fn get_or_create(&self, name: Arc<String>) -> Arc<Group> {
        self.0.lock()
            .unwrap()
            .entry(name.clone()) //"Gets the given key’s corresponding entry in the map for in-place manipulation."
            .or_insert_with(|| Arc::new(Group::new(name))) //insert result of the closure if empty
            .clone()
    }
}

