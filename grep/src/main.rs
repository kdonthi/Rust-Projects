use std::io;
use std::env;

fn is_substring_of(small_string: &str, big_string: &str) -> bool {
    let big_string_len = big_string.len();
    let small_string_len = small_string.len();

    if &big_string_len < &small_string_len {
        return false;
    }
    let mut firstindex = 0;
    let mut lastindex = small_string_len;
    let mut substring: &str;
    while lastindex <= big_string_len {
        substring = &big_string[firstindex..lastindex];
        if &big_string[firstindex..lastindex] == small_string { //why do we put address of pointer?
            return true;
        }
        //println!("{}", substring);
        firstindex += 1;
        lastindex += 1;
    }
    return false;


}

fn main() {
    let args: Vec<String> = env::args().collect(); //collect turns an iterator into a collection
    assert_eq!(args.len(), 2);
    let substring = &args[1];
    //let mut startindex;
    //let mut endindex;
    let mut line = String::new();
    let mut result: bool;
    let mut counter = 1;
    while (true) {
        io::stdin().read_line(&mut line);
        result = match (line.is_empty()) {
            true => break,
            false => false,
        };
        if (result == true) {
            break;
        }
        //println!("{}", line);
        //startindex = 0;
        //endindex = substring.len();
            //line = &text_holder[startindex..endindex];
        if (is_substring_of(substring, &line) == true) {
            println!("{}", line.trim());
        }
        line.clear();
    }
}
