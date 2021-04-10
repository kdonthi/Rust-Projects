//use std::num::ParseIntError;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::vec::Vec;
use std::fs::File;
use std::io::Error;
fn get_request(request: &mut TcpStream) -> String {
    //let outgoing = request.accept();
    /*let mut outgoing = match outgoing {
        Ok((TcpStream, socketaddr)) => TcpStream,
        Err(err) => panic!("{}", err),
    };*/
    let mut array: [u8; 1024] = [0; 1024];
    //let mut smartpointer = Box(array);
    request.read(&mut array); //why does read need a mutable reference
    let stringReturn :String = String::from_utf8_lossy(&array).to_string(); //lossy converts from array while wo lossy converts from vector
    //what is the point of doing [..] if we are not having a string?
    println!("{}", stringReturn);
    return stringReturn;
}
fn strcmp(str1: &str, str2: &str) -> bool { 
    let str1b = str1.as_bytes();
    let str2b = str2.as_bytes();
    println!("Str1: {:?}, Str2: {:?}", str1b, str2b);
    println!("Str1: {:?}, Str2: {:?}", str1, str2);
    if str1.len() != str2.len() {
        return false;
    }
    for i in 0..str1.len() {
        println!("Str1: {}, Str2: {}", str1b[i], str2b[i]);
        if str1b[i] != str2b[i] {
            return false;
        }
    }
    return true;
}

fn main () -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:7878");
    let listener = match listener {
        Ok(TCPListener) => TCPListener,
        Err(err) => panic!("{:?}", err),
    };
    let mut streamCopy;
    let mut counter: i8;

    for stream in listener.incoming() { //any way to make request mut?
        streamCopy = stream;
        println!("{:?}", streamCopy);
        let mut streamCopy = match streamCopy {
            Ok(strm) => strm,
            Err(err) => panic!("Error in listening!"),
        };
        counter = 0;
        let mut requestString = get_request(&mut streamCopy);
        let mut responseVector: Vec<char> = Vec::new();
        let mut responseString: String;
        let mut htmlfile;
        let mut contents = String::new();
        for i in requestString.split("\n") {
            println!("{}", i);
            
            for (counter, j) in (i.split(" ")).enumerate() {
                if counter == 2 {
                    if (strcmp(i, "GET / HTTP/1.1\r")) {
                        htmlfile = File::open("correctHTML.html");
                        htmlfile?.read_to_string(&mut contents);
                        responseVector.extend(j.trim().chars());
                        responseVector.extend(" 200 OK\r\nContent-Type: text/html\n".chars());
                    }
                    else { 
                        htmlfile = File::open("errorHTML.html");
                        htmlfile?.read_to_string(&mut contents);
                        responseVector.extend(j.trim().chars());
                        responseVector.extend(" 404 NOT FOUND\r\nContent-Type: text/html\n".chars());
                    }
                    //responseVector.extend(j.trim().chars()); //putting first line in HTML response
                    break;
                }
            }
            break;
        }
        responseVector.extend(contents.chars());
        match streamCopy.write(&responseVector.iter().collect::<String>().as_bytes()[..]) {
            Ok(n) => n,
            Err(err) => panic!("{}", err),
        };
        /*match streamCopy.write(&responseVector.iter().collect::<String>().as_bytes()[..]) {
            Ok(n) => n,
            Err(err) => panic!("{}", err),
        };*/
        //streamCopy.write(&responseVector[..]);
    }
    Ok(())
}
