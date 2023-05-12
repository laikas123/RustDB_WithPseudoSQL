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
use std::collections::HashMap;
use std::collections::BTreeMap;






fn main() {
    
    let mut input = String::new();
    input = get_user_input(input);

    let input_split = input.split_whitespace();

    for word in input_split {
        println!("word = {}", word);
    }

    let my_interpreter = CommandInterpreter::new(input.clone());

    my_interpreter.interpret_command();
  
}
    
pub fn get_user_input(mut input: String) -> String {
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}



#[derive(PartialEq, Debug, Clone, Copy)]
enum ColType {
    RequiredStrCol,
    OptionalStrCol,
    RequiredIntCol,
    OptionalIntCol,
}



struct Db{
    tables: HashMap<String, DbTable>,
}

//although every entry in the table
//is stored as type string, ColType is
//there to keep track of whether int
//to string conversions need to happen
#[derive(Debug)]
struct DbTable {
    name: String,

    //note the indices of each vector 
    //line up with each other


    //column names and types
    int_cols: Vec<(String, ColType)>,
    str_cols: Vec<(String, ColType)>,
    
    //data for each row
    int_rows: Vec<Vec<Option<usize>>>,
    str_rows: Vec<Vec<Option<String>>>,
}



impl DbTable {

    //name = the name of the table
    //metadata = column names and types
    //import_str_rows = rows of str data to import
    //import_int_rows = rows of int data to import
    pub fn new(name: String, metadata: Vec<(String, ColType)>, import_str_rows: Vec<Vec<Option<String>>>, import_int_rows: Vec<Vec<Option<usize>>>) -> Option<Self> {


        //make sure that if rows are being imported that each column is covered
        //and that there's an equal amount of rows from all types
        if import_str_rows.len() != 0 || import_int_rows.len() != 0 {

            println!("Got here");

            //get length of first str row
            let len_str_row = import_str_rows[0].len();

            //make sure all of the str_rows have this same length
            let str_all_same_len = import_str_rows.iter().all(|elem| elem.len() == len_str_row);

            if !str_all_same_len {
                println!("Error, not all str_rows are same length");
                return None;
            }


            //get length of first int row
            let len_int_row = import_int_rows[0].len();

            //make sure all of the int_rows have this same length
            let int_all_same_len = import_int_rows.iter().all(|elem| elem.len() == len_int_row);

            if !int_all_same_len {
                println!("Error, not all int_rows are same length");
                return None;
            }

            //make sure the combined length matches total cols
            if (len_str_row + len_int_row) !=  metadata.len() {
                println!("Error, import data doesn't span all columns");
                return None;
            }

            //not equal total number of rows from each type
            if import_str_rows.len() != import_int_rows.len() {
                println!("Error, different number of rows for each type");
                return None;
            }
            
        }
       

        let mut int_cols = Vec::new();
        let mut str_cols = Vec::new();

        for (col_name, col_type) in metadata {
            if col_type == ColType::RequiredIntCol || col_type == ColType::OptionalIntCol {
                int_cols.push((col_name, col_type));
            }else if col_type == ColType::RequiredStrCol || col_type == ColType::OptionalStrCol{
                str_cols.push((col_name, col_type));
            }
        }


        let mut int_rows = Vec::new();
        let mut str_rows = Vec::new();

        

        for row in import_str_rows {
            let mut i = 0;
            let mut row_vec = Vec::new();
            for col_val in row {
                //If None is trying to be put somewhere it shouldn't abort all
                if col_val.is_none() && str_cols[i].1 == ColType::RequiredStrCol {
                    println!("Error required column {} given None value", str_cols[i].0);
                    return None;
                }
                row_vec.push(col_val);

                i += 1;
            }

            str_rows.push(row_vec);
        }

        for row in import_int_rows {
            let mut i = 0;
            let mut row_vec = Vec::new();
            for col_val in row {
                //If None is trying to be put somewhere it shouldn't abort all
                if col_val.is_none() && int_cols[i].1 == ColType::RequiredIntCol {
                    println!("Error required column {} given None value", int_cols[i].0);
                    return None;
                }
                row_vec.push(col_val);

                i += 1;
            }
            int_rows.push(row_vec);
        }

        Some(DbTable 
        {
            name: name,
            int_cols: int_cols,
            str_cols: str_cols,
            int_rows: int_rows,
            str_rows: str_rows,
        })


    }

    pub fn get_column_index_and_type(&self, col_name: &str) -> Option<(usize, ColType)> {
        
        //first check num cols (note .iter() iterates over reference so no move
        //occurs)
        let mut pos = self.int_cols.iter().position(|elem| elem.0 == col_name);
        if pos.is_some() {
            let ret_index = pos.unwrap();
            return Some((ret_index, self.int_cols[ret_index].1));
        }

        pos = self.str_cols.iter().position(|elem| elem.0 == col_name);
        if pos.is_some() {
            let ret_index = pos.unwrap();
            return Some((ret_index, self.str_cols[ret_index].1));
        }

        return None;



    }


    fn is_required_column(col_type: &ColType) -> bool {
        if *col_type == ColType::RequiredIntCol || *col_type == ColType::RequiredStrCol {
            return true;
        }else{
            return false;
        }
    }

    //returns the new row count if successful
    //otherwise returns -1
    pub fn insert(&mut self, import_str_rows: Vec<Vec<Option<String>>>, import_int_rows: Vec<Vec<Option<usize>>>) -> i32{
        

        //make sure that if rows are being imported that each column is covered
        //and that there's an equal amount of rows from all types
        if import_str_rows.len() != 0 || import_int_rows.len() != 0 {

            println!("Got here");

            //get length of first str row
            let len_str_row = import_str_rows[0].len();

            //make sure all of the str_rows have this same length
            let str_all_same_len = import_str_rows.iter().all(|elem| elem.len() == len_str_row);

            if !str_all_same_len {
                println!("Error, not all str_rows are same length");
                return -1;
            }


            //get length of first int row
            let len_int_row = import_int_rows[0].len();

            //make sure all of the int_rows have this same length
            let int_all_same_len = import_int_rows.iter().all(|elem| elem.len() == len_int_row);

            if !int_all_same_len {
                println!("Error, not all int_rows are same length");
                return -1;
            }

            //make sure the combined length matches total cols
            if len_str_row  != self.str_cols.len() || len_int_row != self.int_cols.len() {
                println!("Error, import data doesn't span all columns");
                return -1;
            }

            //not equal total number of rows from each type
            if import_str_rows.len() != import_int_rows.len() {
                println!("Error, different number of rows for each type");
                return -1;
            }
            
        }

        for row in import_str_rows {
            let mut i = 0;
            let mut row_vec = Vec::new();
            for col_val in row {
                //If None is trying to be put somewhere it shouldn't abort all
                if col_val.is_none() && self.str_cols[i].1 == ColType::RequiredStrCol {
                    println!("Error required column {} given None value", self.str_cols[i].0);
                    return -1;
                }
                row_vec.push(col_val);

                i += 1;
            }

            self.str_rows.push(row_vec);
        }

        for row in import_int_rows {
            let mut i = 0;
            let mut row_vec = Vec::new();
            for col_val in row {
                //If None is trying to be put somewhere it shouldn't abort all
                if col_val.is_none() && self.int_cols[i].1 == ColType::RequiredIntCol {
                    println!("Error required column {} given None value", self.int_cols[i].0);
                    return -1;
                }
                row_vec.push(col_val);

                i += 1;
            }
            self.int_rows.push(row_vec);
        }

        //could have also been self.str_rows.len()
        //but either will do since they are same
        return (self.int_rows.len()).try_into().unwrap();

    }

  


    pub fn pretty_print(&self) {
        println!("\nPRINTING TABLE:\n");
        println!("Table: {}", &self.name);
        println!("Str Cols: \n{:?}", &self.str_cols);
        println!("Str Rows:");
        &self.str_rows.iter().for_each(|elem| { println!("{:?}", elem) });
        println!("Int Cols: \n{:?}", &self.int_cols);
        println!("Int Rows:");
        &self.int_rows.iter().for_each(|elem| { println!("{:?}", elem) });
        println!("\n");
    }



}




#[test]
fn test_insert() {



    let name = "Shoes".to_string();
    let metadata = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![Some("Slippers".to_string())], 
                               vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), Some(33), None], vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, metadata, import_str_rows, import_int_rows).expect("good");

    shoes_table.pretty_print();


    let import_str_rows = vec![vec![Some("Heels".to_string())], 
                               vec![Some("Flats".to_string())]];
    let import_int_rows = vec![vec![Some(40), Some(2), Some(5)], vec![Some(33), Some(70), None]];

    let new_len = shoes_table.insert(import_str_rows, import_int_rows);
    println!("new len {}", new_len);

    assert_eq!(new_len, 4);


    



}


#[test]
fn test_new(){

    let name = "Shoes".to_string();
    let metadata = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![None], 
                               vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), Some(33), None], 
                               vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, metadata, import_str_rows, import_int_rows);

    //should fail because missing required name 
    assert_eq!(shoes_table.is_none(), true);


    let name = "Shoes".to_string();
    let metadata = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), Some(33), None], 
                               vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, metadata, import_str_rows, import_int_rows);

    //should fail because length issue 
    assert_eq!(shoes_table.is_none(), true);


    let name = "Shoes".to_string();
    let metadata = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![Some("Slippers".to_string())], 
                               vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), None, None], 
                               vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, metadata, import_str_rows, import_int_rows);

    //should fail because missing required stock
    assert_eq!(shoes_table.is_none(), true);


    let name = "Shoes".to_string();
    let metadata = vec![("Name".to_string(), ColType::RequiredStrCol), ("Price".to_string(), ColType::RequiredIntCol), ("Stock".to_string(), ColType::RequiredIntCol), ("Discount".to_string(), ColType::OptionalIntCol)];
    let import_str_rows = vec![vec![Some("Slippers".to_string())], 
                               vec![Some("Boots".to_string())]];
    let import_int_rows = vec![vec![Some(12), Some(33), None], vec![Some(33), Some(5), Some(12)]];

    
    let mut shoes_table = DbTable::new(name, metadata, import_str_rows, import_int_rows).expect("good");

    //should pass since passed in clean data
    // assert_eq!(shoes_table.is_none(), false);

    shoes_table.pretty_print();

   


}



//note key words will list all key words found 
//and it will hold them in the exact same order they 
//appear 
// struct Command {
//     keywords

// }





















//used to store information
//about a word from a command
//including the word, the position,
//and whether it's a keyword
//although option would work for 
//KeyWord attrs I believe it's less
//expensive to just create empty 
//keyword attrs for non keywords
#[derive(Debug, Clone)]
struct CommandWordDat {
    name: String,
    keyword: bool,
    attrs: KeywordAttrs,

}

//note Variable means at least one arg,
//but could some number larger than one 
//as well
#[derive(PartialEq, Debug, Clone, Copy)]
enum KeywordNumArgs {
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
struct KeywordAttrs {
    num_args: KeywordNumArgs,
    special_args: Option<Vec<String>>,
    callback_func: Option<fn(Vec<String>) -> bool>,
}


struct CommandInterpreter {
    keywords: HashMap<String, KeywordAttrs>,
    current_columns: Vec<String>,
    current_btreemap: BTreeMap<usize, CommandWordDat>,
    current_database: Option<Db>,
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











