use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::thread;
use commands;

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
            Ok(data) => data,
            Err(e) => {
                println!("Ошибка открытия порта: {}", e);
                return false;
            },
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let address = self.address.clone();

                    thread::spawn(move || {
                        handle_client(address, stream);
                    });
                },
                Err(e) => {
                    println!("Ошибка при запуске сервера: {}", e);
                    return false;
                }
            }
        }

        fn handle_client(address: String, client_stream: TcpStream) {
            /*println!("Подключен неизвестный клиент, ip: {}:{}",
                     reader.get_ref().local_addr().unwrap().ip(),
                     reader.get_ref().local_addr().unwrap().port());*/

            let mut reader = BufReader::new(&client_stream);
            let mut writer = BufWriter::new(&client_stream);

            loop {
                let mut data = String::new();

                let result = {
                    match reader.read_line(&mut data).unwrap() {
                        0 => {
                            println!("Неизвестный клиент был отключен, ip: {}:{}",
                                     reader.get_ref().local_addr().unwrap().ip(),
                                     reader.get_ref().local_addr().unwrap().port());
                            return;
                        },
                        _ => (),
                    }

                    let mut server_stream = TcpStream::connect(&*address).unwrap();

                    println!("Принял данные: {}", data);

                    let data = data.trim();
                    let data: Vec<&str> = data.split_whitespace().collect();

                    match data[0] {
                        "login" => commands::login(&mut writer, &mut server_stream, &data[1..]),
                        "register" => commands::new_account(&data[1..]),
                        _ => false,
                    }
                };

                if !result {
                    println!("Неверная команда");
                } else {
                    writer.write(b"OK").unwrap();
                    writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
                }
            }
        }

        true
    }
}