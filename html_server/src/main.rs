use std::num::ParseIntError;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::vec::Vec;
fn reply(request: &mut TcpStream) -> String {
    //let outgoing = request.accept();
    /*let mut outgoing = match outgoing {
        Ok((TcpStream, socketaddr)) => TcpStream,
        Err(err) => panic!("{}", err),
    };*/
    let mut array: [u8; 1024] = [0; 1024];
    //let mut smartpointer = Box(array);
    request.read(&mut array);
    let stringReturn :String = String::from_utf8_lossy(&array).to_string(); //lossy converts from array while wo lossy converts from vector
    //what is the point of doing [..] if we are not having a string?
    println!("{}", stringReturn);
    return (stringReturn);
}   
fn main () {
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
    let mut requestCopy;
    let mut counter: i8 = 0;
    for request in listener.incoming() {
        requestCopy = request;
        println!("{:?}", requestCopy);
        let mut requestCopy = match requestCopy {
            Ok(stream) => stream,
            Err(err) => panic!("Error in listening!"),
        };
        let requestCopy = reply(&mut requestCopy);
        let responseVector: Vec<char> = Vec::new();
        for i in requestCopy.split("\n") {
            println!("{}", i);
            counter += 1;
            println!("\nCounter: {}", counter);
            if (counter >= 3) {
                break;
            }
        }
        counter = 0;
    }

}
