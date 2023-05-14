//import sibling
use super::db_structures::*;
use super::load_balancer::*;
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
    VariableTwoPlus,
    VariableThreePlusOdd,
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CommandFunctionality {
    NoOp,
    UpdateCols,
    UpdateTable,
    UpdateDb,
    CreateDb,
    CreateTable,
    InsertIntoTable,
}

//meta data about keywords
//e.g. create_db can only have one arg
//
#[derive(Debug, Clone)]
pub struct KeywordAttrs {
    pub num_args: KeywordNumArgs,
    pub command_functionality: CommandFunctionality,
}

#[derive(Debug)]
pub struct CmdInterpreter {
    pub keywords: HashMap<String, KeywordAttrs>,
    pub current_columns: Vec<String>,
    pub current_btreemap: BTreeMap<usize, CommandWordDat>,
}


impl CmdInterpreter {

    pub fn new() -> Self {
        let mut interp: CmdInterpreter = 
        
        CmdInterpreter 
        {
            keywords: Self::create_keywords_map(),
            current_columns: Vec::new(),
            current_btreemap: BTreeMap::new(),
        };

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
                command_btreemap.insert(index, CommandWordDat{name: word.to_string(), keyword: false, attrs: KeywordAttrs{num_args: KeywordNumArgs::Empty,  command_functionality: CommandFunctionality::NoOp}});
            }
            

            index = index+1;
        }

        self.current_btreemap =  command_btreemap;

    }

    pub fn interpret_command(&mut self, database_list: &mut HashMap<String, (StatusFlag, Db)>, current_database: String) -> bool{

        //go through each word in the command
        for (key, val) in &self.current_btreemap{
            
            //if it's a keyword like Select, create_table, etc.
            if val.keyword {

                //get the command attributes
                //and expected number of args
                let attrs = &val.attrs;
                let num_args = attrs.num_args;

                

                //edge case, if the last word is keyword and expects args but there are none
                //this is a panic, need to handle more gracefully though...
                if (*key == self.current_btreemap.len()-1) && num_args != KeywordNumArgs::Empty {
                    //or handle cases where it can be here...
                    println!("if last word is keyword it can't take args");
                    return false;
                }

                //search for index of next keyword
                //everything in between must be args
                //the next keyword must have index in command
                //greater than current
                let index_next_keyword = match self.current_btreemap.iter().position(|elem| elem.0 > key && elem.1.keyword ) {
                    Some(index) => index,
                    _ => self.current_btreemap.len(),
                };

                println!("Position next keyword {}", index_next_keyword);

                let expected_num_args = index_next_keyword - key - 1;

                if !Self::validate_number_of_args(num_args, index_next_keyword - *key - 1) {
                    println!("Error invalid number of args for {} expected {:?} got {}", val.name, num_args, expected_num_args);
                    println!("Note, for create_db spaces are not allowed");
                    println!("Note, for create_table total args must be odd number since table name + 2*number of cols (since each call has two params col name and type)");
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

                match attrs.command_functionality {
                    
                    CommandFunctionality::CreateDb => {
                        //note right now this really updates a Db
                        //but to actually persist the Db in the filesystem
                        //need to actually create files to handle this...
                        // self.current_database.name = args[0].clone();
                        // self.current_database.tables = HashMap::new();
                        // self.current_database.status = DbStatus::Selected;

                        database_list.insert(args[0].clone(), (StatusFlag::Clean, Db{name: args[0].clone(), tables: HashMap::new(), status: DbStatus::Empty}));

                            
                    },
                    CommandFunctionality::CreateTable => {
                        
                        

                        if !database_list.contains_key(&current_database) {
                            println!("Can't create table for database {}, cannot find this database", current_database);
                            return false;
                        }

                        //get database corresponding to clients currently selected database
                        let database = &mut database_list.get_mut(&current_database).expect("already checked valid").1;


                        //table name is first argument
                        let name = args[0].clone();

                       //make sure no existing table with that name exists
                        if database.tables.contains_key(&name){
                            println!("Error table with name {} already exists in Db {}", &name, current_database);
                            return false;
                        }
                        

                        //the valid column types
                        //first char is required "R" or optional "O"
                        //second char is string "S" or int "I"
                        let valid_types = vec!["RS".to_string(), "OS".to_string(), "RI".to_string(), "OI".to_string()];

                        let mut col_names_types = Vec::new();

                        //uses a window of 2 e.g. [args[1], args[2]], [args[3], args[4]]
                        for col_name_type in args[1..].chunks(2) {
                            println!("{:?}", col_name_type);

                            //make sure the column type is one of the expected types
                            let matched_index = valid_types.iter().position(|elem| *elem == col_name_type[1]);

                            if matched_index.is_none() {
                                println!("Error unknown column type {}", &col_name_type[1]);
                                return false;
                            }else{
                                col_names_types.push((col_name_type[0].clone(), string_to_coltype(&col_name_type[1])));
                            }

                        }

                        let new_table = DbTable::new(name.clone(), col_names_types, Vec::new(), Vec::new());

                        match new_table {
                            Some(table) => {
                                println!("Created table {} successfully", &name);
                                database.tables.insert(name, table);
                            },
                            _ => {
                                println!("Error could not create table");
                                return false;
                            },
                        }

                        

                        
                    },
                    //user must supply vals for each column, even optional columns
                    //for none use special symbol ?
                    CommandFunctionality::InsertIntoTable => {


                        if !database_list.contains_key(&current_database) {
                            println!("Can't create table for database {}, cannot find this database", current_database);
                            return false;
                        }

                        //get database corresponding to clients currently selected database
                        let database = &mut database_list.get_mut(&current_database).expect("already checked valid").1;


                        //table name is first argument
                        let name = args[0].clone();

                        //make sure the table exists
                        if !database.tables.contains_key(&name){
                            println!("Error table with name {} doesn't exist in Db {}", &name, current_database);
                            return false;
                        }

                        let table = &mut database.tables.get(&name).expect("alrady checked existence");

                        let str_len = table.str_cols.len();
                        let int_len = table.int_cols.len();

                        //there should be one argument for each column in the table
                        let expected_num_args = str_len + int_len;

                        if args[1..].len() != expected_num_args {
                            println!("Error incorrect number of args, got {}, expected {}", args[1..].len(), expected_num_args);
                            return false;
                        }


                        let mut int_args = Vec::new();

                        for i in (1+str_len)..1+expected_num_args {
                            match args[i].parse::<usize>() {
                                Ok(parsed_int) => {
                                    int_args.push(parsed_int);
                                },
                                _ => {
                                    println!("Error could not parse int for argument {}", i);
                                    return false;
                                },
                            }
                        }

                        // (*table).insert(vec![args[1..table.str_cols.len()].to_vec()], vec![int_args]);

                        // self.current_database.insert_into_table(name, vec![args[1..table.str_cols.len()].to_vec()], vec![int_args]);

                        // let mutable_table = self.current_database.table_mut(name);

                        match database.table_mut(name.clone()) {
                            Some(mutable_table) => {
                                mutable_table.insert(vec![args[1..1+str_len].to_vec()], vec![int_args]);
                            },
                            _ => {
                                println!("Error could not get mutable reference to table {}", name);
                                return false;
                            },
                        }



                    },
                    _ => (), 
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
            KeywordNumArgs::VariableTwoPlus => arg_count >= 2, //for insert_into takes at least (table_name, col val(s))
            KeywordNumArgs::VariableThreePlusOdd => arg_count >= 3 && arg_count % 2 == 1,   //this is for create_table where you need at least (table name, col1 name, col1 type)
                                                                                            //it also needs to be an odd number because table name is 1 + you always need col name and type
                                                                                            //so it's always 1 + 2*x where x is the number of columns so this must always be odd 
                                                                                        
        }
    }


    pub fn create_keywords_map() -> HashMap<String, KeywordAttrs> {
        let mut keywords = HashMap::new();        
        //select can have variable number of args but at least 1, also * has special meaning
        // keywords.insert("select".to_string(), KeywordAttrs{num_args: KeywordNumArgs::Variable, command_functionality: CommandFunctionality::UpdateCols});
        //from typically just takes a table name so 1 arg, * is also special arg here meaning all tables, but I'll ignore that for now
        keywords.insert("from".to_string(), KeywordAttrs{num_args: KeywordNumArgs::Single, command_functionality: CommandFunctionality::UpdateTable});
        // //takes three arguments usually e.g. where "x = y", and = must be present so it's considered special
        // keywords.insert("where".to_string(), KeywordAttrs{num_args: KeywordNumArgs::Triple, special_args: Some(vec!["="])});
        // //
        keywords.insert("insert_into".to_string(), KeywordAttrs{num_args: KeywordNumArgs::VariableTwoPlus, command_functionality: CommandFunctionality::InsertIntoTable});
        // keywords.insert("values".to_string());  
        keywords.insert("create_table".to_string(), KeywordAttrs{num_args: KeywordNumArgs::VariableThreePlusOdd, command_functionality: CommandFunctionality::CreateTable});
        keywords.insert("create_db".to_string(), KeywordAttrs{num_args: KeywordNumArgs::Single, command_functionality: CommandFunctionality::CreateDb});
        // keywords.insert("table".to_string());  
        // keywords.insert("union".to_string());
        // keywords.insert("and".to_string());
        keywords
        
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


    pub fn pretty_print(&self) {
        // println!("{:?}", self.keywords);
        println!("{:?}", self.current_columns);
        // println!("{:?}", self.current_btreemap);
        // println!("{:?}", self.current_database);
        // self.current_database.pretty_print_tables();

    }


    #[test]
    fn test_command_to_btreemap() {

        let mut test_tree = command_to_btreemap("select name from users;");

    }


}











