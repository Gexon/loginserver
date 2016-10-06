use std::net::TcpStream;
use std::io::BufWriter;
use std::io::prelude::*;
use dbqury as db;


pub fn login(writer: &mut BufWriter<&TcpStream>, _server_stream: &mut TcpStream, args: &[&str]) -> bool {
    if args.len() != 2 {
        return false
    }

    //println!("Готов к передаче данных, имя {} пароль {}", args[0], args[1]);

    if !db::check_name(args[0]) {
        println!("Пользователь {} не зарегистрирован. Провожу регистрацию нового пользователя.", args[0]);
        db::add_account(args[0], args[1], 0);
        let _ = writer.write(b"REG\n");
        writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
        return true
    } else {
        let hash = db::get_mdhash(args[0]);
        if hash == args[1] {
            let _ = writer.write(b"OK\n");
            writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
            return true
        } else {
            let _ = writer.write(b"ERR_LOGIN\n");
            writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
            return true
        }
    }
    //    match server_stream.write(format!("login {} {}\n", args[0], args[1]).as_bytes()) {
    //        Ok(_) => true,
    //        Err(_) => false,
    //    }
    true
}

pub fn chat(writer: &mut BufWriter<&TcpStream>, _server_stream: &mut TcpStream, args: &[&str]) -> bool {
    if args.len() < 1 {
        return false
    }

    let merged: String = args.iter()
        .flat_map(|s| s.chars().chain(" ".chars()))
        .collect();
    /*let mut owned_string: String = "hello ".to_owned();
    let borrowed_string: &str = "world";
    owned_string.push_str(borrowed_string);
    println!("{}", owned_string);*/
    let cmsg: &str = "chat_all";
    let msg: &str = &merged;//args[0..];
    let emsg: &str = "\n";
    let smsg: String = format!("{} {}{}", cmsg, msg, emsg);
    println!("Высылаю данные клиенту: {}", smsg);
    let _ = writer.write(smsg.as_bytes());
    writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
    true
}

pub fn new_account(args: &[&str]) -> bool {
    if args.len() != 2 {
        return false
    }

    println!("Регистрирую новый аккаунт, имя {} пароль {}", args[0], args[1]);
    db::add_account(args[0], args[1], 0);

    true
}

pub fn _check_auth(name: &str, _mdhash: &str) {
    db::get_mdhash(name);
}