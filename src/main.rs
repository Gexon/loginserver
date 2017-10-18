//#[macro_use]
//extern crate getopts;
extern crate time;

mod server;
mod commands;
mod dbqury;

use server::LoginServer;

const  VERSION: &'static str = "8.3";

fn main() {
    //let hname: &str = "192.168.0.3";
    //let hname: &str = "185.40.31.100";
    let hname: &str = "0.0.0.0";
    let pname: &str = "6656";

    //    // let args = match application::parse_command_line(){
    //        Some(data) => data,
    //        None => return,
    //    };


    let mut server = LoginServer::new(hname, pname);


    server.start();
}