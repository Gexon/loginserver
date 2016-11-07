#[macro_use] extern crate getopts;
extern crate time;

mod server;
mod commands;
mod dbqury;

use server::LoginServer;

//static VERSION: &'static str = "7.6";
const  VERSION: &'static str = "7.7";

fn main() {
    let hname: &str = "192.168.0.131";
    //let hname: &str = "194.87.237.144";
    let pname: &str = "6656";

    //    // let args = match application::parse_command_line(){
    //        Some(data) => data,
    //        None => return,
    //    };


    let mut server = LoginServer::new(hname, pname);


    server.start();
}