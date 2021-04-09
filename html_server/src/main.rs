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

fn main () -> Result<(), Error> {
    //FtpListner -> bind onto the IP address we want to listen on //so where is the server being created?
    //is an ip address just a port where a connection CAN arrive at (like a port is a place a ship can dock?)
    //is a server just something that listens to incoming connections on an IP address and responds?
    //whenever you have a ?, it needs to be in a function that returns a Result((), whatever type inside error)
    //run cargo check to get cool error checking!
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
            Ok(stream) => stream,
            Err(err) => panic!("Error in listening!"),
        };
        counter = 0;
        let mut requestString = get_request(&mut streamCopy);
        let mut responseVector: Vec<char> = Vec::new();
        for i in requestString.split("\n") {
            if counter == 0 {
                for (counter, j) in (i.split(" ")).enumerate() {
                    //println!("Counter: {}, Val: {}", counter ,j);
                    if counter == 2 {
                        println!("{:?}", j.trim().chars());
                        responseVector.extend(j.trim().chars()); //putting first line in HTML response
                        break;
                    }
                }
                //panic!("G5 and we live!");
                //responseVector.extend(i.split(" ")[2].chars()); //putting first line in HTML response
                responseVector.extend(" 200 OK\r\n".chars());
                responseVector.extend("Content-Type: text/html\n".chars());
            }
            else {
                responseVector.extend(i.chars()); //putting headers
                responseVector.extend("\n".chars());
            }
            counter += 1;
            //println!("\nCounter: {}", counter);
        }
        let htmlfile = File::open("sampleHTML.html");
        let mut contents = String::new();
        htmlfile?.read_to_string(&mut contents);
        responseVector.extend(contents.chars());
        println!("{:?}", responseVector.iter().collect::<String>());
        match streamCopy.write(&responseVector.iter().collect::<String>().as_bytes()[..]) {
            Ok(n) => n,
            Err(err) => panic!("{}", err),
        };
        //streamCopy.write(&responseVector[..]);
    }
    Ok(())
}
