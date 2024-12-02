use std::{collections::{BTreeMap, HashMap}, fmt::Display, hash::Hash, str::FromStr};

#[derive(Clone,Eq,PartialEq,Hash)]
enum IntNum {
    I32(i32),
    I64(i64)

}

#[derive(Clone, Eq, PartialEq, Hash)]
enum JsonTypes {
    Null,
    String(String),
    Bool(bool),
    Int(IntNum),
    Vec(Vec<JsonTypes>),
    Dict(BTreeMap<String,JsonTypes>),
    
}
#[derive(Clone)]
struct JsonObject {
    contents: HashMap<String,JsonTypes>
}


fn ret_bool(input: String) -> Result<JsonTypes,String>
{
    if input.starts_with("true") {
        Ok(JsonTypes::Bool(true))
    } else if input.starts_with("false") {
        Ok(JsonTypes::Bool(false))
    } else {
        Err("Could not perform conversion from string to bool.".to_string())
    }
}

fn ret_vec(input: String) -> Result<JsonTypes,JsonTypes>
{
    if input.starts_with("[") {
        let split = input.split_terminator(",");
        let mut filtered_vec = Vec::<JsonTypes>::new(); 
        let mut n = 0;
       
        for character in split {
            n += 1;
            if character.contains(['1','2','3','4','5','6','7','8','9']) {
                filtered_vec[n] = JsonTypes::Int(IntNum::I32(character.parse::<i32>().unwrap()))
                // automatically converts into i32 by default
            
            } if character.starts_with("true") {
                filtered_vec[n] = JsonTypes::Bool(true)
            } else {
                filtered_vec[n] = JsonTypes::Bool(false)
            // all these are checks for possible conversion
        }
    }
        Ok(JsonTypes::Vec(filtered_vec))
    
    } else {
        Err(JsonTypes::Vec(vec![JsonTypes::Null]))
    }
}
// all other non implemented types can be converted because they implement the ToString trait



impl JsonObject {
    fn from_line(&self,content: String) -> Self {
        //! Constructs a `JsonObject` from a single JSON key value pair.
        let mut parsed = HashMap::new();
        let mut n = 0;
        for character in content.chars() {
            n += 1;
            if character == ':' {
                
                parsed.insert(content[..n].to_string(),"blah");
            }

        }
        Self {
            contents:parsed
        }
    }
}