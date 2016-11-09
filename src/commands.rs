use std::net::TcpStream;
use std::io::BufWriter;
use std::io::prelude::*;
use time::{Timespec, Duration};
use time;
use dbqury as db;
use VERSION;

pub fn login(writer: &mut BufWriter<&TcpStream>, _server_stream: &mut TcpStream, args: &str) -> bool {
    let args: Vec<&str> = args.splitn(2, ' ').collect();

    //println!("Готов к передаче данных, имя {} пароль {}", args[0], args[1]);

    if !db::check_name(args[0]) {
        println!("Пользователь {} не зарегистрирован. Провожу регистрацию нового пользователя.", args[0]);
        db::add_account(args[0], args[1], 0);
        let _ = writer.write(b"REG\n");
        writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
        return true
    } else {
        // не проверяем токен авторизации
        let hash = db::get_mdhash(args[0]);
        if hash == args[1] {
            let token_i64: i64 = new_auth_token(args[0]);
            let answer: String = format!("OK {}\n", token_i64);
            let _ = writer.write(answer.as_bytes());
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

// проверяем версию клиентской части.
pub fn hello(writer: &mut BufWriter<&TcpStream>, args: &str) -> bool {
    let args = args.trim();
    //println!("args{}", args);
    //println!("VERSION {}", VERSION);
    if args == VERSION {
        let _ = writer.write(b"hello\n");
        writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
    } else {
        let _ = writer.write(b"old_bich\n");
        writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
    }

    true
}

pub fn new_account(args: &str) -> bool {
    let args: Vec<&str> = args.splitn(2, ' ').collect();
    if args.len() != 2 {
        return false
    }

    println!("Регистрирую новый аккаунт, имя {} пароль {}", args[0], args[1]);
    db::add_account(args[0], args[1], 0);

    true
}

// проверяем авторизацию и ее актуальность
pub fn _check_auth(name: &str, in_hash: &str) -> bool {
    let hash = db::get_mdhash(name);
    if hash == in_hash {
        // добавить проверку токена авторизации
        return true
    }
    false
}

// пишем новый токен авторизации
pub fn new_auth_token(name: &str) -> i64 {
    let current_time = time::get_time();
    // let localtime = time::now();
    // let localtime = localtime.asctime();
    // println!("Unixtime: {}, localtime: {}", current_time.sec, localtime);
    // let stime = time::strftime("{}", &localtime);
    // let stime = time::strftime("{}", current_time.sec);

    // вычисляем новый токен или обновляем существующий.
    // let now = current_time.sec;
    // в токен пишем пока в чистом виде время окончания токена.
    let token: Timespec = current_time + Duration::days(1);
    // let stoken = format!("{}", token);
    // записываем токен в БД
    db::set_token(name, token.sec);
    token.sec
}

// проверяем актуальность токена
pub fn _check_token(name: &str) -> bool {
    // получаем из БД токен
    let token: i64 = db::_get_token(name);
    // расшифровываем
    // получаем текущую дату и сравниваем не истек ли токен.
    let current_time = time::get_time();
    if token > current_time.sec {
        return true
    }

    false
}