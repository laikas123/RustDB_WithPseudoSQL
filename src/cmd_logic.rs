use super::db_structures::*;
use std::collections::HashMap;
use std::collections::BTreeMap;


//used to store information
//about a word from a command
//including the word, the position,
//and whether it's a keyword
//although option would work for 
//KeyWord attrs I believe it's less
//expensive to just create empty 
//keyword attrs for non keywords
#[derive(Debug, Clone)]
pub struct CommandWordDat {
    pub name: String,
    pub keyword: bool,
    pub attrs: KeywordAttrs,

}

//note Variable means at least one arg,
//but could some number larger than one 
//as well
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KeywordNumArgs {
    Empty,
    Single,
    Double,
    Tripe,
    Variable,
}

//this is for meta data about
//keywords like how many arguments
//can come after a keyword
//call back functions for validation
//etc
//for example select in my simplified
//case can include some number of keywords
//e.g. 1 or more columns or * for all
//special args are args that have special
//meaning e.g. * for select
//TODO 
//the arguments need callback functions...
//e.g. the first argument might always need
//to be a table name in the case of insert_into
//in this case the callback function would be to 
//check if the table exists
//and for columns the callback function would be
//check if the column exists within the table
//which brings up the point of as a command is
//interepreted there needs to be additional data
//stored such as the table being acted upon....
//
//after some thinking it seems the best 
//way to implement the callbacks is to have the 
//same signature for them all, which I'm going to 
//let be Fn(&Vec<String>) -> bool
//
//the reason for vec is you might be checking
//multiple args at once, and in all cases I can
//think of you're gonna just validate whether 
//something is valid so a bool should be fine
#[derive(Debug, Clone)]
pub struct KeywordAttrs {
    pub num_args: KeywordNumArgs,
    pub special_args: Option<Vec<String>>,
    pub callback_func: Option<fn(Vec<String>) -> bool>,
}


pub struct CommandInterpreter {
    pub keywords: HashMap<String, KeywordAttrs>,
    pub current_columns: Vec<String>,
    pub current_btreemap: BTreeMap<usize, CommandWordDat>,
    pub current_database: Option<Db>,
}


impl CommandInterpreter {

    pub fn new(command: String) -> Self {
        let mut interp: CommandInterpreter = 
        
        CommandInterpreter 
        {
            keywords: Self::create_keywords_map(),
            current_columns: Vec::new(),
            current_btreemap: BTreeMap::new(),
            current_database: None,
        };

        interp.set_btreemap(command);

        interp

    }

    //todo add some sort of error handling for when an invalid command is given...
    //for now just get commands up and running...
    pub fn set_btreemap(&mut self, command: String) {

        let mut command_btreemap = BTreeMap::new();

        let command_iter = command.split_whitespace();

        let mut index = 0;

        for word in command_iter {
            if self.is_keyword(word){
                command_btreemap.insert(index, CommandWordDat{name: word.to_string(), keyword: true, attrs: self.get_keyword_attrs(word.to_string())});
            }else{
                command_btreemap.insert(index, CommandWordDat{name: word.to_string(), keyword: false, attrs: KeywordAttrs{num_args: KeywordNumArgs::Empty, special_args: None, callback_func: None}});
            }
            

            index = index+1;
        }

        self.current_btreemap =  command_btreemap;

    }

    pub fn interpret_command(&self) -> bool{

        for (key, val) in &self.current_btreemap{
            if val.keyword {

                

                let attrs = &val.attrs;
        
            
                let num_args = attrs.num_args;

                

                //edge case
                if *key == self.current_btreemap.len() && num_args != KeywordNumArgs::Empty {
                    //or handle cases where it can be here...
                    println!("if last word is keyword it can't take args");
                    return false;
                }

                //search for index of next keyword
                //everything in between must be args
                //the next keyword must have key val
                //greater than current and 
                let index_next_keyword = match self.current_btreemap.iter().position(|elem| elem.0 > key && elem.1.keyword ) {
                    Some(index) => index,
                    _ => self.current_btreemap.len(),
                };

                println!("Position next keyword {}", index_next_keyword);

                let expected_num_args = index_next_keyword - key - 1;

                if !Self::validate_number_of_args(num_args, index_next_keyword - *key - 1) {
                    println!("Error invalid number of args for {} expected {:?} got {}", val.name, num_args, expected_num_args);
                    return false;
                }

                let mut args = Vec::new();

                for i in (*key + 1)..index_next_keyword {
                    match self.current_btreemap.get(&i) {
                        Some(val_ref) => args.push(val_ref.name.to_string()),
                        _ => panic!("Couldn't traverse tree given valid tree"),
                    }
                }

                println!("args are {:?}", args);

                match attrs.callback_func {
                    Some(function) => function(args),
                    _ => true, 
                };
                
                

            }
                   
            
        }

        // println!("position next keyword {:?}", self.current_btreemap.iter().position(|elem| elem.1.name == "from"));

        println!("cols are: {:?}", self.current_columns);

        true

    }

    pub fn validate_number_of_args(arg_type: KeywordNumArgs, arg_count: usize) -> bool {
        match arg_type {
            KeywordNumArgs::Empty => arg_count == 0,
            KeywordNumArgs::Single => arg_count == 1,
            KeywordNumArgs::Double => arg_count == 2,
            KeywordNumArgs::Tripe => arg_count == 3,
            KeywordNumArgs::Variable => arg_count >= 1,
        }
    }


    pub fn create_keywords_map() -> HashMap<String, KeywordAttrs> {
        let mut keywords = HashMap::new();        
        //select can have variable number of args but at least 1, also * has special meaning
        keywords.insert("select".to_string(), KeywordAttrs{num_args: KeywordNumArgs::Variable, special_args: Some(vec!["*".to_string()]), callback_func: Some(Self::set_columns)});
        //from typically just takes a table name so 1 arg, * is also special arg here meaning all tables, but I'll ignore that for now
        keywords.insert("from".to_string(), KeywordAttrs{num_args: KeywordNumArgs::Single, special_args: None, callback_func: Some(Self::set_columns)});
        // //takes three arguments usually e.g. where "x = y", and = must be present so it's considered special
        // keywords.insert("where".to_string(), KeywordAttrs{num_args: KeywordNumArgs::Triple, special_args: Some(vec!["="])});
        // //
        // keywords.insert("insert_into".to_string());
        // keywords.insert("values".to_string());  
        // keywords.insert("create".to_string());        
        // keywords.insert("table".to_string());  
        // keywords.insert("union".to_string());
        // keywords.insert("and".to_string());
        keywords
        
    }

    pub fn set_columns(name: Vec<String>) -> bool {
        println!("got called");
        return true;
    }

    pub fn get_keyword_attrs(&self, keyword: String) -> KeywordAttrs {

        return self.keywords.get(&keyword.to_lowercase()).expect("Error when fetching keyword").clone();

    }

    pub fn is_keyword(&self, word: &str) -> bool {
        //perform lowercase conversion before checking
        //so keywords aren't missed based on that
        if self.keywords.contains_key(&word.to_lowercase()){
            return true;
        }else{
            return false;
        }
    }



    #[test]
    fn test_command_to_btreemap() {

        let mut test_tree = command_to_btreemap("select name from users;");

    }


}











