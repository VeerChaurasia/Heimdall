use std::{io::{self, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let mut a=String::new();
    io::stdin().read_line(&mut a).unwrap();
    a=a.trim().to_string();
    if a=="Listen"{
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        println!("Listening on 127.0.0.1:8000");


    }
    else if a=="Connect"{
        let mut stream= TcpStream::connect("127.0.0.1:8000").unwrap();
        println!("Connected to 127.0.0.1:8000");
        stream.write_all(b"Hello Heimdal!\n").unwrap();
    }

}
