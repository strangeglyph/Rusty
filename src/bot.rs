use std;
import std::*;


import io;
import io::reader;
import io::writer;
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

    /**
     * Creates a new bot that connects to host:port with the given nick
     */
    new(host:str, port:uint) {
        
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
    
    /**
     * Reads a line from the server. Blocks until the read is completed.
     * 
     * # Returns
     * The received text
     */
    fn read_line() -> str {

        let read = sock as reader;
        let recv = read.read_line().trim();
        
        #info[ "[→] %s", recv ];
        
        ret recv;
    }
    
    /**
     * Sends a raw command to the IRC server. Appends linefeeds.
     * 
     * # Arguments
     * * `text` -- The command to send
     */
    fn send_raw(text: str) {

        let write = sock as writer;

        write.write_str(text + "\r\n");
        write.flush();

        #info[ "[←] %s", text ];
    }
    
    /**
     * Sends a private message to a person or channel.
     * 
     * # Arguments
     * * `target` -- The target that should receive the message
     * * `message` -- The message to send
     */
    fn send_msg(target: str, message: str) {
        send_raw("PRIVMSG " + target + " :" + message);
    }
    
    /**
     * Sends a notice to a person or channel.
     * 
     * # Arguments
     * * `target` -- The target that should receive the message
     * * `message` -- The message to send.
     */
    fn send_notice(target: str, message: str) {
        send_raw("NOTICE " + target + " :" + message);
    }
}





// String utilities
fn println(anything: to_str) {
    io::println(anything.to_str());
}

fn print(anything: to_str) {
    io::print(anything.to_str());
}
