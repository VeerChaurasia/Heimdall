use std::{io::{self, BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main() {
    println!("Type 'Listen' to wait for connections, or 'Connect' to connect to a peer:");
    let mut a=String::new();
    io::stdin().read_line(&mut a).unwrap();
    a=a.trim().to_string();
    if a=="Listen"{
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        println!("Listening on 127.0.0.1:8000");
        for streams in listener.incoming(){
            let mut stream=streams.unwrap();
            let mut stream_writer = stream.try_clone().unwrap();
            println!("Connection Established");
            std::thread::spawn(move || {
                let mut reader = BufReader::new(stream);
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(0) => {
                            println!("Client disconnected.");
                            break;
                        }
                        Ok(_) => {
                            println!("Received: {}", line.trim());
                        }
                        Err(e) => {
                            eprintln!("Error reading from client: {}", e);
                            break;
                        }
                    }
                }
            });
            std::thread::spawn(move || {
                loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let response = input.trim();
                    stream_writer.write_all(response.as_bytes()).unwrap();
                    stream_writer.write_all(b"\n").unwrap();
                }
            });
        }


    }
    else if a=="Connect"{
         let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    println!("Connected to 127.0.0.1:8000");

    // Clone the stream for the read thread
    let stream_clone = stream.try_clone().unwrap();
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stream_clone);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    println!("Server disconnected.");
                    break;
                }
                Ok(_) => {
                    println!("Received: {}", line.trim());
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });
        loop{
            let mut input=String::new();
            io::stdin().read_line(&mut input).unwrap();
            let message=input.trim();
            if message=="Exit"{
                println!("Breaking Connection");
                break;

            }
            stream.write_all(message.as_bytes()).unwrap();
            stream.write_all(b"\n").unwrap();
        }
    }

}
