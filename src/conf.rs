use std;

import result::result;
import io;
import io::reader;

import std::map;


export load;
export save;

fn load(filename: str) -> result<map::hashmap<str, ~[str]>, str> {
    
    let res = io::file_reader(filename);
    
    if (res.is_err()) { 
        ret result::err(res.get_err())
    }
    
    let conf = map::str_hash();
    let reader = res.get();
    
    ret result::ok(conf)
}

fn save(conf: map::hashmap<str, ~[str]>, filename: str) -> result<str, str> {
    
}
