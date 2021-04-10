use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::vec::Vec;
use std::fs::File;
use std::io::Error;
use threadpool::ThreadPool;

fn get_request(request: &mut TcpStream) -> String {
    let mut array: [u8; 1024] = [0; 1024];
    request.read(&mut array).expect("Read Failed"); //why does read need a mutable reference
    let string_return: String = String::from_utf8_lossy(&array).to_string(); //lossy converts from array while wo lossy converts from vector
    return string_return;
}

fn handle_stream(stream: Result<TcpStream, Error>) -> Result<(), Error> {
    let stream_copy = stream;
    let mut stream_copy = match stream_copy {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    };
    let request_string = get_request(&mut stream_copy);
    let mut response_vector: Vec<char> = Vec::new();
    let htmlfile;
    let mut contents = String::new();
    for i in request_string.split("\n") {
        
        for (counter, j) in (i.split(" ")).enumerate() {
            if counter == 2 {
                if i == "GET / HTTP/1.1\r" {
                    htmlfile = File::open("correctHTML.html");
                    htmlfile?.read_to_string(&mut contents)?;
                    response_vector.extend(j.trim().chars());
                    response_vector.extend(" 200 OK\r\nContent-Type: text/html\n".chars());
                }
                else { 
                    htmlfile = File::open("errorHTML.html");
                    htmlfile?.read_to_string(&mut contents)?;
                    response_vector.extend(j.trim().chars());
                    response_vector.extend(" 404 NOT FOUND\r\nContent-Type: text/html\n".chars());
                }
                break;
            }
        }
        break;
    }
    response_vector.extend(contents.chars());
    match stream_copy.write(&response_vector.iter().collect::<String>().as_bytes()[..]) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    };
    Ok(())
}

fn main () -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:7878");
    let listener = match listener {
        Ok(tcp_listener) => tcp_listener,
        Err(err) => panic!("{:?}", err),
    };
    let pool = ThreadPool::new(100);

    for stream in listener.incoming() { 
        pool.execute(|| {
            handle_stream(stream).expect("Stream not handled");
            });
    }
    pool.join();
    Ok(())
}
