use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::thread;

use commands;
use VERSION;
//use std::mem;
//use std::slice;

pub struct LoginServer {
    address: String,
    //    reader: BufReader<TcpStream>,
}

impl LoginServer {
    pub fn new(hostname: &str, port: &str) -> LoginServer {
        let address = format!("{}:{}", hostname, port);
        //        let stream = TcpStream::connect(&*address).unwrap();

        LoginServer {
            address: address,
            //            reader: BufReader::new(stream),
        }
    }

    pub fn start(&mut self) -> bool {
        let listener = match TcpListener::bind(&*self.address) {
            Ok(data) => {
                println!("LS>Логин-сервер запущен. V{}", VERSION);
                data
            },
            Err(e) => {
                println!("LS>Ошибка открытия порта: {}", e);
                return false;
            },
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let address = self.address.clone();

                    thread::spawn(move || {
                        handle_client(&address, &stream);
                    });
                },
                Err(e) => {
                    println!("LS>Ошибка при подключении клиента: {}", e);
                    return false;
                }
            }
        }

        fn handle_client(address: &str, client_stream: &TcpStream) {
            println!("LS>Подключен неизвестный клиент, ip: {}",
                     client_stream.peer_addr().unwrap().ip());

            let mut reader = BufReader::new(client_stream);
            let mut writer = BufWriter::new(client_stream);

            loop {
                let mut data = String::new();

                let result = {
                    if let 0 = reader.read_line(&mut data).unwrap() {
                        println! ("LS>Неизвестный клиент был отключен, ip: {}:{}",
                                  client_stream.peer_addr().unwrap().ip(),
                                  client_stream.peer_addr().unwrap().port());
                        return;
                    }

                    let mut server_stream = TcpStream::connect(address).unwrap();

                    println!("LS>Принял данные: {}", data.trim());

                    let data = data.trim();
                    //let data: Vec<&str> = data.split_whitespace().collect();
                    let data: Vec<&str> = data.splitn(2, ' ').collect();


                    match data[0] {
                        "login" => commands::login(&mut writer, &mut server_stream, data[1]),
                        "register" => commands::new_account(data[1]),
                        "hello" => commands::hello(&mut writer, data[1]),
                        _ => false,
                    }
                };

                if !result {
                    println!("LS>Неверная команда");
                    let _ = writer.write(b"ERR_COMMAND\n");
                    writer.flush().unwrap();
                } else {
                    //let answer = String::from("OK");
                    //                    let size_dat = answer.len();

                    // превращаем размер в байты
                    //                    let size: usize = size_dat;
                    //                    let csize: *const usize = &size;
                    //                    let bp: *const u8 = csize as *const _;
                    //                    let bs: &[u8] = unsafe {
                    //                        slice::from_raw_parts(
                    //                            bp,
                    //                            mem::size_of::<usize>()
                    //                        )
                    //                    };

                    //                    println!("Размер данных answer {}", answer.len());
                    //                    println!("Содержимое size_dat {}", size_dat);
                    //                    println!("Размер байтмассива bs {}", bs.len());

                    //let _ = writer.write(bs);   // шлем 8 байт размер данных.
                    //                    let _ = writer.write(answer.as_bytes());
                    //let _ = writer.write(b"OK\n");
                    //writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток






                    /*
                        // some bytes, in a vector
                        let sparkle_heart = vec![240, 159, 146, 150];

                        // We know these bytes are valid, so we'll use `unwrap()`.
                        let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

                        assert_eq!("💖", sparkle_heart);
                    */
                }
            }
        }

        true
    }
}