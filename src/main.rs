extern crate clipboard;

use std::env;
use std::fs;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

struct Credentials {

    // Credential object that will store all the credentials in separate vectors

    cred: Vec<String>,
    username: Vec<String>,
    password: Vec<String>,
    text: String,
}

fn load_creds() -> Credentials {

    // Loads VAULT file and load it into an instance of Credentials structure

    let text = fs::read_to_string("/Users/rumbleftw/Documents/Codes/smhash/src/VAULT").expect("VAULT file missing!");

    let mut cred: Vec<String> = Vec::new();
    let mut username: Vec<String> = Vec::new();
    let mut password: Vec<String> = Vec::new();

    for line in text.lines() {
        let items: Vec<&str> = line.split(',').collect();
        cred.push(items[0].to_string());
        username.push(items[1].to_string());
        password.push(items[2].to_string());
    }

    let cred_instance = Credentials { cred: cred, username: username, password: password, text: text};
    return cred_instance;
}

fn search(query: String, creds: &Credentials) -> i32 {

    // Searches for param query in creds and return its index

    let mut results: Vec<usize> = Vec::new();
    for (idx, cred) in creds.cred.iter().enumerate() {
        if cred.to_string().contains(&query) {
            results.push(idx);
        }
    }
    if results.len() == 0 {
        return -1;
    }
    return results[0] as i32;
}

fn get(query: String, creds: Credentials, verbose: bool) {

    // Prints the credential and copies the password to the clipboard

    let idx: i32 = search(query, &creds);
    if idx < 0 {
        println!("Credentials not found! Please try again :D");
        return;
    }
    let pass: String;
    if verbose {
        pass = creds.password[idx as usize].clone();
    }
    else {
        pass = "*".repeat(creds.password[idx as usize].len());
    }
    println!("\nCredentials for {} - ID/Username: {}, Password: {} [Copied to clipboard]\n", creds.cred[idx as usize], creds.username[idx as usize], pass);
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(creds.password[idx as usize].to_owned()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args recieved: {:?}", args);
    let query = &args[1];
    if query == "dump" {
        let creds = load_creds();
        println!("{}", creds.text);
    }
    else if query == "search" {
        let creds = load_creds();
        search("net".to_string(), &creds);
    }

    else if query == "get" {
        let creds = load_creds();
        let mut v: bool = false;
        if args.contains(&"-v".to_string()) {
            v = true;
        }
        get(args[2].to_string(), creds, v);
    }
}