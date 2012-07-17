use std;
import std::*;

// core imports
import io;
import io::println;
import io::{reader, reader_util};
import io::{writer, writer_util};

import to_str;
import to_str::to_str;

import task;

import result;


// std imports
import ip = net::ip;
import socket = net::tcp;
import net::tcp::tcp_socket_buf;

import uv::iotask;
import uv::iotask::iotask;

// bot imports
import conf;
import conf::conf;


fn main() {
    
    #debug[ "Entering main method" ];
    
    let conf = conf::load(~"bot.conf").get();
    let bot = Bot(conf);
    
    #info[ "Connected" ];
    
    while bot.is_connected() {
        bot.read_line();
    }
    
    println(~"Done");
}




class Bot {
    
    priv {
        let conf: conf::map_conf;
        let sock: @tcp_socket_buf;
        let mut connected: bool;
    }

    // Parser breaks with doc strings over constructors
    // /**
    //  * Creates a new bot that connects to host:port
    //  * 
    //  * # Arguments
    //  * * `conf` -- The configuration for the bot
    //  */
    new(conf: conf::map_conf) {
        
        self.conf = conf;
        let host = conf.get_first(~"host").get();
        let port = conf.get_uint(~"port").get();
        
        #info[ "Getting ip for host %s", host ];
        let ip = ip::v4::parse_addr(host);
        let task = iotask::spawn_iotask(task::builder());
        
        
        #info[ "Connecting socket (%s:%u)", host, port ];
        let res = socket::connect(ip, port, task);
        
        if res.is_err() {
            #error[ "Failed to connect to target: %?", res.get_err() ];
            // UGLY, but needed - flow analysis else thinks the sock is not set
            let unbuffered = result::unwrap(res);
            self.sock = @socket::socket_buf(unbuffered);
            self.connected = false;
            fail;   // Will have failed already
        }
        
        
        #debug[ "Unwrapping and buffering" ];
        let unbuffered = result::unwrap(res);
        self.sock = @socket::socket_buf(unbuffered);
        self.connected = true;
    }
    
    /**
     * Reads a line from the server. Blocks until the read is completed.
     * 
     * # Returns
     * The received text
     */
    fn read_line() -> ~str {

        if (!self.connected) { fail ~"Disconnected" };

        let read = self.sock as reader;
        let recv = read.read_line();
        println( #fmt["[→] %s", recv] );
        
        ret recv;
    }
        
    /**
     * Sends a raw command to the IRC server. Appends linefeeds.
     * 
     * # Arguments
     * * `text` -- The command to send
     */
    fn send_raw(text: ~str) {

        if (!self.connected) { fail ~"Disconnected" };

        let writer = self.sock as writer;
        writer.write_str(text + ~"\r\n");
        writer.flush();

        println( #fmt["[←] %s", text] );
    }
    
    /**
     * Sends a private message to a person or channel.
     * 
     * # Arguments
     * * `target` -- The target that should receive the message
     * * `message` -- The message to send
     */
    fn send_msg(target: ~str, message: ~str) {
        self.send_raw(~"PRIVMSG " + target + ~" :" + message);
    }
    
    /**
     * Sends a notice to a person or channel.
     * 
     * # Arguments
     * * `target` -- The target that should receive the message
     * * `message` -- The message to send.
     */
    fn send_notice(target: ~str, message: ~str) {
        self.send_raw(~"NOTICE " + target + ~" :" + message);
    }
    
    
    /**
     * Disconnects the bot from the server.
     * All subsequent reads and writes will fail.
     * 
     * # Arguments
     * * `reason` -- The reason to give the server for the disconnect
     */
    fn disconnect(reason: ~str) {
        self.send_raw(~"QUIT :" + reason);
        self.connected = false;
    }
    
    fn is_connected() -> bool {
        ret self.connected;
    }
}
