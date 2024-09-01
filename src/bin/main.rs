use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(100) {     //incoming() will give an iterator over the connections being received on our listener in the form of a tcp stream. take(2) creates a new iterator yielding the first n element, here n=2.
        let stream = stream.unwrap();

        //pool.execute is going to take a closure & give it to a thread in the pool to execute.
        pool.execute( || {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
    // here, the main function ends then our ThreadPool goes out of scope & its drop method is called,
    // In ThreadPool drop method, we send the termination message to all the workers.

}

fn handle_connection (mut stream: TcpStream) {
    let mut buffer = [0; 1024];     // 

    stream.read(&mut buffer).unwrap();      //Populate the buffer with data from the stream.
                                            // read() returns Result Type, unwrap() to panic if Error. 

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));  // from_utf8_lossy() converts a slice of bytes to string including invalid characters.

    let get = b"GET / HTTP/1.1\r\n";    //prefixing w `b` will give a byte array representing our string.
    let sleep = b"GET /sleep HTTP/1.1\r\n";  //request line that specifies a GET request to a sleep route.

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } 
    else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
        
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    
    // Content-Length header specifies the amount of bytes w're returning in the message body, and getting that number from contents.len()
    let response = format! ("{}\r\nContent-Length: {}\r\n\r\n{}",
                            status_line,
                            contents.len(),
                            contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();     //flush() will wait until all bytes are written to the connection.  

}


