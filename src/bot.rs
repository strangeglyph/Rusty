use std;
import std::*;

// core imports
import io;
import io::println;
import io::{reader, reader_util};
import io::{writer, writer_util};

import str;
import vec;

import to_str;
import to_str::to_str;

import task;

import result;
import option;


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
    bot.run();
    
    
    println(~"Done");
}

enum Event {
    CONNECT
}

type Listener = {event: Event, handle: fn(data: ~[~str])};


class Bot {
    
    priv {
        let conf: conf::map_conf;
        let sock: @tcp_socket_buf;
        let mut connected: bool;
        let mut listeners: ~[Listener];
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
        self.listeners = ~[];
        
        let host = self.conf.get_first(~"host").get();
        let port = self.conf.get_uint(~"port").get();
            
        #info[ "Getting ip for host %s", host ];
        let ip = ip::v4::parse_addr(host);
        let task = iotask::spawn_iotask(task::builder());
            
        #info[ "Connecting socket (%s:%u)", host, port ];
        let res = socket::connect(ip, port, task);
            
        #debug[ "Unwrapping and buffering" ];
        let unbuffered = result::unwrap(res);
        self.sock = @socket::socket_buf(unbuffered); 
            
        self.connected = true;  
    }
    
    fn run() {
        
        self.identify();
        self.change_nick(self.conf.get_first(~"nick").get());
        
        while self.is_connected() {
            
            let mut recv = self.read_line().trim();
            
            if recv.starts_with(~":") { 
                recv = recv.slice(1, recv.len()); 
            }
            
            if recv.starts_with(~"PING") {
                self.send_raw(~"PONG :" + recv.slice(6, recv.len()));
            } else {
                // Handle stuff
            }
        }
    }
    
    fn register_listener(event: Event, fn(data: ~[~str])) {
        vec::push(self.listeners, {event, data});
    }
    
    fn fire_event(event: Event, data: ~[~str]) {
        for self.listeners.each |listener| {
            listener.handle(data);
        }
    }
    
    
    /**
     * Reads a line from the server. Blocks until the read is completed.
     * 
     * # Returns
     * The received text
     */
    fn read_line() -> ~str {

        if (!self.is_connected()) { fail ~"Disconnected" };

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

        if (!self.is_connected()) { fail ~"Disconnected" };

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
    
    /**
     * Tells if the bot is connected to the server.
     * 
     * # Returns
     * * `true` if the bot is connected, `false` otherwise
     */
    pure fn is_connected() -> bool {
        self.connected
    }
    
    fn identify() {
        let user = self.conf.get_first(~"user").get();
        let desc = self.conf.get_first(~"desc").get();
        
        self.send_raw(~"USER " + user + " * * :" + desc); 
    }
    
    fn change_nick(nick: ~str) {
        self.send_raw(~"NICK :" + nick);
    }
    
    fn join(room: ~str) {
        self.send_raw(~"JOIN :" + room);
    }
    
    fn part(room: ~str) {
        self.send_raw(~"PART :" + room);
    }
}
