# Rust-Projects

Rust is a programming language that is used for safety, especially focused on preventing double frees through ownership rules. I created some projects to further develop my knowledge: 

1. ```weight-on-mars``` is a project where you type in your weight in Newtons on Earth and you get your weight in Newtons on Mars. I used standard input, error handling, and strings in this project.

2. ```grep``` is an emulation of the Linux ```grep``` command. I used error handling using match, slices, string references, and a substring algorithm in this project.

3. ```html_server``` is a project where I created a multi-threaded HTML server to serve HTML files on the IP address `127.0.0.1:7878`. I created a thread pool, instantiated a TcpListener, and wrote to a TcpStream by using a buffer to create this project. After running it using `cargo run`, try accessing the default IP address and try accessing a subdomain of the IP address (like `127.0.0.1:7878/hello`); you should see two different HTML files. If you want to see different outputs, you can edit the HTML files in the `html-server/src` directory:
<ol>
  <li> correctHTML.html => the file that is served when you access the default page </li>
  <li> errorHTML.html => the file that is served when you access a subdirectory </li> <br>
</ol>

You can run all of these programs by going into the directory you are interested in, and typing in ```cargo run```. If that doesn't work, and the project doesn't use any crates like `grep`, you can go inside the ```src``` folder in the directory and compile the file manually with ```rustc main.rs``` and execute it using ```./main```.

