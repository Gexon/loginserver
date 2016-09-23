use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::thread;

pub struct LoginServer {
    address: String,
    reader: BufReader<TcpStream>,
}

impl LoginServer {
    pub fn new(hostname: &str, port: &str) -> LoginServer {
        let address = format!("{}:{}", hostname, port);
        let stream = TcpStream::connect(&*address).unwrap();

        LoginServer {
            address: address,
            reader: BufReader::new(stream),
        }
    }

    pub fn start(&mut self) {
        let listener = match TcpListener::bind(&*self.addres) {
            Ok(data) => data,
            Err(e) => {
                println!("Ошибка открытия порта: {}", e);
                return false;
            },
        };

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move || {
                        handle_client(&mut stream)
                    });
                },
                Err(e) => {
                    println!("Ошибка при запуске сервера: {}", e);
                    return false;
                }
            }
        }

        fn handle_client(stream: &mut TcpStream) {
            let reader = BufReader::new(stream);

            loop {
                let mut data = String::new();
                match reader.read_line(&mut data).unwrap() {
                    0 => {
                        println!("Неизвестный клиент был отключен, ip: {}:{}",
                                 reader.get_ref().local_addr().unwrap().ip(),
                                 reader.get_ref().local_addr().unwrap().port());
                        return;
                    },
                    _ => (),
                }

                println!("Передача данных: {}", data);
            }
        }


        //        loop {
        //            let _ = self.reader.get_mut().write(b"Dotakiller\n");
        //        }
    }
}