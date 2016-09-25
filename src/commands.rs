use std::net::TcpStream;
use std::io::prelude::*;
use SQLQuery as db;


pub fn login(stream: &mut TcpStream, server_stream: &mut TcpStream, args: &[&str]) -> bool {
    if args.len() != 2 {
        return false
    }

    println!("Готов к передаче данных, имя {} пароль {}", args[0], args[1]);


    match server_stream.write(format!("login {} {}\n", args[0], args[1]).as_bytes()) {
        Ok(_) => true,
        Err(_) => false,
    }

    //true
}

pub fn new_account(args: &[&str]) -> bool {
    if args.len() != 2 {
        return false
    }

    db::add_account(args[0], args[1], 0);

    true
}

pub fn check_auth(name: &str, mdhash: &str) {
    db::get_mdhash(name);
}