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

fn save(conf: map::hashmap<str, ~[str]>, filename: str) -> result<str, str> {
    ret result::ok("OK");
}

#[test]
fn testLoad() {
    #error[ "=== Start load() test" ];
    load("bot.conf");
    #error[ "=== End load() test" ];
}

#[test]
fn testCompleteness() {
    #error[ "=== Start completeness test" ];
    let result = load("bot.conf");
    
    assert result.is_ok();
    
    let conf = result.get();
    
    assert conf.contains_key("nick");
    assert conf.contains_key("user");
    assert conf.contains_key("desc");
    assert conf.contains_key("host");
    assert conf.contains_key("port");
    assert conf.contains_key("chan");
    #error[ "=== End completeness test" ];
}

#[test]
fn testMultiItems() {
    #error[ "=== Start multi-item test" ];
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
    
    #error[ "=== End multi-item test" ];
}
