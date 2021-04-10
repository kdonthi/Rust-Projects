# Rust-Projects

Rust is a programming language that is used for safety, especially focused on preventing double frees through ownership rules. Because I am interested in the blockchain space, and Rust is very prevalent in the space, I decided to make a few projects to expand my knowledge.

```weight-on-mars``` is a project where you type in your weight in Newtons on Earth and you get your weight in Newtons on Mars. I used standard input, error handling, and strings in this project.

```grep``` is a project where I am trying to emulate the Linux ```grep``` command. I am learning about and using error handling using match, slices, string references, and a substring algorithm in this project.

```html_server``` is a project where I created a multi-threaded HTML server to serve HTML files on the IP address `127.0.0.1:7878`. I used a thread pool, TcpListeners, TcpStreams, and  After running it, try accessing the IP address and also try accessing a subdomain of the IP address (`127.0.0.1:7878/hello`); you will see two different HTML files. 

You can run both of these programs by going into the directory you are interested in, and typing in ```cargo run```. If that doesn't work, you can just go inside the ```src``` folder in the directory and compile the file manually with ```rustc main.rs``` and execute it using ```./main```.

