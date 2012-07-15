use std;

import result::result;
import io;
import io::reader;
import io::reader_util;
import vec;

import std::map;
import std::map::*;


export load;
export save;

/**
 * Loads a configuration file into a hashmap.
 * 
 * # Arguments
 * * `filename` -- The path to the configuration file.
 * 
 * # Returns
 * The hashmap if everything went well, an error otherwise.
 */
fn load(filename: str) -> result<map::hashmap<str, ~[str]>, str> {
    
    #debug[ "Opening conf file '%s'", filename ];
    
    let res = io::file_reader(filename);
    
    if (res.is_err()) { 
        ret result::err(copy res.get_err())
    }
    
    let conf = map::str_hash();
    let reader = res.get();
    
    loop {
        let line = reader.read_line().trim();
        
        if reader.eof() { break; }
        if line == "" || line.starts_with("#") { again; }
        
        #debug[ "conf read line from %s: '%s'", filename, line ];
        let parts = line.split_char('=');
        
        if parts.len() < 2 {
            ret result::err( #fmt["Incomplete line in configuration file %s: '%s'", filename, line] )
        }
        
        let mut values: ~[str] = ~[];
        
        if conf.contains_key(parts[0].trim().to_lower()) {
            values = conf.get(parts[0].trim().to_lower());
        }
        
        values += vec::map(parts[1].split_char(';'), |value| {
            value.trim()
        });
        
        conf.insert(parts[0].trim().to_lower(), values);
    }
    
    ret result::ok(conf)
}

/**
 * Saves a configuration hashmap to a file.
 * 
 * # Arguments
 * * `conf` -- The map to save
 * * `filename` -- The path to the file where the map should be saved
 * 
 * # Returns
 * result::ok if everything went okay, result::err if an error occurred
 */
fn save(conf: map::hashmap<str, ~[str]>, filename: str) -> result<str, str> {
    ret result::ok("OK");
}

fn vec_to_str(vec: ~[str]) -> str {
    
    let mut res = "";
    let mut first = true;
    
    for vec.each |part| {
        if part.trim() == "" { again; } // Don't add empty strings
        
        if (!first) { res += ";"; }     // Add separators
        else        { first = false; }  

        res += part.trim();             // Remove superfluous whitespace
    };
    
    ret res;
}

#[test]
fn test_load() {
    load("bot.conf");
}

#[test]
fn test_completeness() {
    let result = load("bot.conf");
    
    assert result.is_ok();
    
    let conf = result.get();
    
    assert conf.contains_key("nick");
    assert conf.contains_key("user");
    assert conf.contains_key("desc");
    assert conf.contains_key("host");
    assert conf.contains_key("port");
    assert conf.contains_key("chan");
}

#[test]
fn test_multi_items() {
    let result = load("bot.conf");
    let conf = result.get();
    let chans = conf.get("chan");
    
    assert chans.len() == 6;
    assert vec::contains(chans, "#a");
    assert vec::contains(chans, "#b");
    assert vec::contains(chans, "#c");
    assert vec::contains(chans, "#d");
    assert vec::contains(chans, "#e");
    assert vec::contains(chans, "#f");
}

#[test]
fn test_vec2str() {
    let vec1 = ~["a","b","c","d","e"];
    let str1 = "a;b;c;d;e";
    let vec2 = ~["ab", "cde"];
    let str2 = "ab;cde";
    let vec3 = ~["     abc    ", "     de"];
    let str3 = "abc;de";
    let vec4 = ~["abcde"];
    let str4 = "abcde";
    
    assert vec_to_str(vec1) == str1;
    assert vec_to_str(vec2) == str2;
    assert vec_to_str(vec3) == str3;
    assert vec_to_str(vec4) == str4;
}
