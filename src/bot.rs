use std;
import std::*;

// core imports
import io;
import io::{reader, reader_util};
import io::{writer, writer_util};

import to_str;
import to_str::to_str;

import task;

import result;


// std imports
import ip = net::ip;
import socket = net::tcp;

import uv::iotask;
import uv::iotask::iotask;




fn main() {
    let bot = Bot("100.100.100.100", 6667);
    #info[ "Done" ];
}

class Bot {
    
    let sock: @socket::tcp_socket_buf;

    // /**
    //  * Creates a new bot that connects to host:port
    //  * 
    //  * # Arguments
    //  * * `host` -- The host name of the target
    //  * * `port` -- The target port
    //  */
    new(host:str, port:uint) {
        
        let ip = ip::v4::parse_addr(host);
        let task = iotask::spawn_iotask(task::builder());
        
        let res = socket::connect(ip, port, task);
        
        if res.is_err() {
            #error[ "Failed to connect to target: %?", res.get_err() ];
            // UGLY, but needed - flow analysis else thinks the sock is not set
            let unbuffered = result::unwrap(res);
            self.sock = @socket::socket_buf(unbuffered);
            fail;   // Will have failed already
        }
        
        let unbuffered = result::unwrap(res);
        self.sock = @socket::socket_buf(unbuffered);
    }
    
    /**
     * Reads a line from the server. Blocks until the read is completed.
     * 
     * # Returns
     * The received text
     */
    fn read_line() -> str {

        let recv = self.read().trim();
        
        #info[ "[→] %s", recv ];
        
        ret recv;
    }
    
    priv {
        fn read() -> str {
            
            let mut buf = ~[];
            
            loop {
                let ch = self.sock.read_byte();
                if ch == -1 || ch == 10 { break; }  // End of stream or \n
                vec::push(buf, ch as u8);
            }
            ret str::from_bytes(buf);
        }
    }
    
    /**
     * Sends a raw command to the IRC server. Appends linefeeds.
     * 
     * # Arguments
     * * `text` -- The command to send
     */
    fn send_raw(text: str) {

        self.sock.write_str(text + "\r\n");
        self.sock.flush();

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
        self.send_raw("PRIVMSG " + target + " :" + message);
    }
    
    /**
     * Sends a notice to a person or channel.
     * 
     * # Arguments
     * * `target` -- The target that should receive the message
     * * `message` -- The message to send.
     */
    fn send_notice(target: str, message: str) {
        self.send_raw("NOTICE " + target + " :" + message);
    }
}
