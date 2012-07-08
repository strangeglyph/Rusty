use std;
import std::*;


import io;
import to_str;
import to_str::to_str;
import task;
import result;

import ip = net::ip;
import socket = net::tcp;
import uv::iotask;
import uv::iotask::iotask;




fn main() {
    let bot = Bot("100.100.100.100", 6667);
    println(0 as to_str);
}

class Bot {
    
    let sock: socket::tcp_socket_buf;

    new(host: str, port:uint) {
        
        let ip = ip::v4::parse_addr(host);
        let task = iotask::spawn_iotask(task::builder());
        
        let res = socket::connect(ip, port, task);
        
        if res.is_err() {
            #error[ "Failed to connect to target: %?", res.get_err() ];
            // UGLY, but needed - flow analysis else thinks the sock is not set
            let unbuffered = result::unwrap(res);
            self.sock = socket::socket_buf(unbuffered);
            fail;
        }
        
        let unbuffered = result::unwrap(res);
        self.sock = socket::socket_buf(unbuffered);
    }
}





// String utilities
fn println(anything: to_str) {
    io::println(anything.to_str());
}

fn print(anything: to_str) {
    io::print(anything.to_str());
}
