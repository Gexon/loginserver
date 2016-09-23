extern crate getopts;
//mod application;
mod server;
mod commands;
use server::LoginServer;

fn main() {
    let hname: &str = "192.168.0.3";
    let pname: &str = "6656";

//    // let args = match application::parse_command_line(){
//        Some(data) => data,
//        None => return,
//    };

    let mut server = LoginServer::new(hname, pname);

    server.start();
}