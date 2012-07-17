use std;

import result::result;
import io;
import io::{reader, reader_util};
import io::{writer, writer_util};
import io::file_flags;
import int;
import vec;

import std::map;
import std::map::*;


export load;
export save;
export map_conf;
export conf;


type map_conf = {backend: map::hashmap<~str, ~[~str]>};

iface conf {
    fn get(key: ~str) -> result<~[~str], ()>;
    fn get_default(key: ~str, default: ~[~str]) -> ~[~str];
    fn get_first(key: ~str) -> result<~str, ()>;
    fn get_int(key: ~str) -> result<int, ()>;
    fn get_uint(key: ~str) -> result<uint, ()>;
    
    fn set(key: ~str, options: ~[~str]) -> bool;
    fn append(key: ~str, options: ~[~str]);
    
    fn _backend() -> map::hashmap<~str, ~[~str]>;
}

impl conf for map_conf {
    
    fn get(key: ~str) -> result<~[~str], ()> {
        if self.backend.contains_key(key.trim().to_lower()) {
            result::ok(self.backend.get(key.trim().to_lower()))
        } else {
            result::err(())
        }
    }
    
    fn get_default(key: ~str, default: ~[~str]) -> ~[~str] {
        let contents = self.get(key);
        
        if contents.is_err() {
            copy default
        } else {
            contents.get()
        }
    }
    
    fn get_first(key: ~str) -> result<~str, ()> {
        let contents = self.get(key);
        
        if contents.is_err() { 
            result::err(()) 
        } else if contents.get().len() < 1 { 
            result::err(()) 
        } else {
            result::ok(contents.get()[0])
        }
    }
    
    fn get_int(key: ~str) -> result<int, ()> {
        let first = self.get_first(key);
        
        if first.is_err() {
            ret result::err(());
        }
        
        let as_int = int::from_str(first.get());
        
        if as_int.is_none() {
            result::err(())
        } else {
            result::ok(as_int.get())
        }
    }
    
    fn get_uint(key: ~str) -> result<uint, ()> {
        let first = self.get_first(key);
        
        if first.is_err() {
            ret result::err(());
        }
        
        let as_uint = uint::from_str(first.get());
        
        if as_uint.is_none() {
            result::err(())
        } else {
            result::ok(as_uint.get())
        }
    }
    
    fn set(key: ~str, options: ~[~str]) -> bool {
        self.backend.insert(key.trim().to_lower(), options)
    }
    
    fn append(key: ~str, options: ~[~str]) {
        let mut existing = ~[];
        
        if self.backend.contains_key(key.trim().to_lower()) {
            existing = self.backend.get(key.trim().to_lower());
        }
        
        existing += options;
        
        self.set(key, existing);
    }
    
    fn _backend() -> map::hashmap<~str, ~[~str]> {
        self.backend
    }
}

fn conf() -> map_conf {
    {backend: map::str_hash()}
}


/**
 * Loads a configuration file into a hashmap.
 * 
 * # Arguments
 * * `filename` -- The path to the configuration file.
 * 
 * # Returns
 * The hashmap if everything went well, an error otherwise.
 */
fn load(filename: ~str) -> result<map_conf, ~str> {
    
    #debug[ "Opening conf file '%s'", filename ];
    
    let res = io::file_reader(filename);
    
    if (res.is_err()) { 
        ret result::err(copy res.get_err())
    }
    
    let conf = {backend: map::str_hash()};
    let reader = res.get();
    
    loop {
        let line = reader.read_line().trim();
        
        // Break when end of file is reached, skip empty lines and comments
        if reader.eof() { break; }
        if line == ~"" || line.starts_with(~"#") { again; }
        
        #debug[ "conf read line from %s: '%s'", filename, line ];
        
        let parts = line.split_char('=');
        if parts.len() < 2 {
            ret result::err( #fmt["Incomplete line in configuration file %s: '%s'", filename, line] )
        }
        
        let key = parts[0].trim().to_lower();
        conf.append(key, vec::map(parts[1].split_char(';'), |value| {
            value.trim()
        }));
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
fn save(conf: conf, filename: ~str) -> result<(), ~str> {
    
    let flags = ~[io::create, io::truncate];
    let result = io::mk_file_writer(filename, flags);
    
    if result.is_err() {
        ret result::err(result.get_err());
    }
    
    let writer = result.get();
    for conf._backend().each |key, value| {
        writer.write_str( #fmt["%s=%s\r\n", key, vec_to_str(value)] );
    };
    
    writer.flush();
    
    ret result::ok(());
}

/**
 * Converts a vector of strings to a string suitable to be saved to a
 * configuration file.
 * 
 * # Arguments
 * * `vec` -- The vector to join
 * 
 * # Returns
 * The vector, joined into a single string.
 */
fn vec_to_str(vec: ~[~str]) -> ~str {
    
    let mut res = ~"";
    let mut first = true;
    
    for vec.each |part| {
        if part.trim() == ~"" { again; } // Don't add empty strings
        
        if (!first) { res += ~";"; }     // Add separators
        else        { first = false; }  

        res += part.trim();             // Remove superfluous whitespace
    };
    
    ret res;
}

#[test]
fn test_load() {
    load(~"bot.conf");
}

#[test]
fn test_completeness() {
    let result = load(~"bot.conf");
    
    assert result.is_ok();
    
    let conf = result.get();
    
    assert conf.get(~"nick").is_ok();
    assert conf.get(~"user").is_ok();
    assert conf.get(~"desc").is_ok();
    assert conf.get(~"host").is_ok();
    assert conf.get(~"port").is_ok();
    assert conf.get(~"chan").is_ok();
}

#[test]
fn test_multi_items() {
    let result = load(~"bot.conf");
    let conf = result.get();
    let chans = conf.get(~"chan").get();
    
    assert chans.len() == 6;
    assert vec::contains(chans, ~"#a");
    assert vec::contains(chans, ~"#b");
    assert vec::contains(chans, ~"#c");
    assert vec::contains(chans, ~"#d");
    assert vec::contains(chans, ~"#e");
    assert vec::contains(chans, ~"#f");
}

#[test]
fn test_vec2str() {
    let vec1 = ~[~"a",~"b",~"c",~"d",~"e"];
    let str1 = ~"a;b;c;d;e";
    let vec2 = ~[~"ab", ~"cde"];
    let str2 = ~"ab;cde";
    let vec3 = ~[~"     abc    ", ~"     de"];
    let str3 = ~"abc;de";
    let vec4 = ~[~"abcde"];
    let str4 = ~"abcde";
    
    assert vec_to_str(vec1) == str1;
    assert vec_to_str(vec2) == str2;
    assert vec_to_str(vec3) == str3;
    assert vec_to_str(vec4) == str4;
}
