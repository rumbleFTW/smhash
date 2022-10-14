use hex;
use sha2::{Sha512, Digest};
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

fn init() {

    // Initializing the MASTER and VAULT files

    if std::fs::metadata("/Users/rumbleftw/Documents/Codes/smhash/src/VAULT").is_ok() || std::fs::metadata("/Users/rumbleftw/Documents/Codes/smhash/src/MASTER").is_ok()  {
        println!("User files detected, please purge all the previous files by running $ smhash purge all");
        return;
    }
    std::fs::File::create("/Users/rumbleftw/Documents/Codes/smhash/src/VAULT").expect("Couldn't create VAULT!");
    std::fs::File::create("/Users/rumbleftw/Documents/Codes/smhash/src/VAULT").expect("Couldn't create VAULT!");
    println!("Enter a new smhash Master Password:");
    let passwd1 = rpassword::read_password().unwrap();
    println!("Re-enter new smhash Master Password:");
    let passwd2 = rpassword::read_password().unwrap();
    if passwd1 == passwd2 {
        let mut hasher = Sha512::new();
        hasher.update(passwd1);
        let hashed_pass = hex::encode(hasher.finalize());
        fs::write("/Users/rumbleftw/Documents/Codes/smhash/src/MASTER", hashed_pass).expect("Could not update MASTER file :/");
    }
    println!("All set! Add a new credential by running $ smhash add");
}

fn purge(all: bool) {

    // Purges all files related to the program

    if all {
        std::fs::remove_file("/Users/rumbleftw/Documents/Codes/smhash/src/MASTER").expect("Couldn't purge MASTER!");
        std::fs::remove_file("/Users/rumbleftw/Documents/Codes/smhash/src/VAULT").expect("Couldn't purge VAULT!");
    }
    else {
        std::fs::remove_file("/Users/rumbleftw/Documents/Codes/smhash/src/VAULT").expect("Couldn't purge VAULT!");
    }
    println!("Purge successful!");
}

fn authenticate() -> bool {

    // Authenticates the master password

    let hashed_master = fs::read_to_string("/Users/rumbleftw/Documents/Codes/smhash/src/MASTER").expect("MASTER file missing!");
    println!("Enter your smhash Master Password:");
    let passwd = rpassword::read_password().unwrap();
    let mut hasher = Sha512::new();
    hasher.update(passwd);
    let hashed_pass = hex::encode(hasher.finalize());
    return hashed_pass == hashed_master;
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

fn import(path: &String) {
    if !std::fs::metadata(&path).is_ok() {
        println!("Invalid path");
        return;
    }
    let data: String = fs::read_to_string(&path).expect("Unexpected error!");
    fs::write("/Users/rumbleftw/Documents/Codes/smhash/src/VAULT", data).expect("Unexpected error!");
    println!("Import successful!");
}

fn add() {
    let mut payload: String = fs::read_to_string("/Users/rumbleftw/Documents/Codes/smhash/src/VAULT").expect("VAULT file missing! :/");
    println!("Enter the Credential service:");
    println!("Enter the Username/ID:");
    println!("Enter the Password:");
    payload = payload + &"\nHalo".to_string();
    fs::write("/Users/rumbleftw/Documents/Codes/smhash/src/VAULT", payload).expect("Could not update VAULT file :/");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args recieved: {:?}", args);
    let query = &args[1];
    if query == "dump" {
        if authenticate() {
            let creds = load_creds();
            println!("{}", creds.text);
        }
        else {
            println!("Invalid Master Password!");
        } 
    }

    // else if query == "search" {
    //     let creds = load_creds();
    //     search("net".to_string(), &creds);
    // }

    else if query == "get" {
        if authenticate() {
            let creds = load_creds();
            let mut v: bool = false;
            if args.contains(&"-v".to_string()) {
                v = true;
            }
            get(args[2].to_string(), creds, v);
        }
        else {
            println!("Invalid Master Password!");
        }
    }

    else if query == "add" {
        if authenticate() {
            add();
        }
        else {
            println!("Invalid Master Password!");
        }
    }

    else if query == "purge" {
        if authenticate() {
            if args.contains(&"all".to_string()) {
                purge(true);
            }
            else {
                purge(false);
            }
        }
        else {
            println!("Invalid Master Password!");
        }
    }

    else if query == "init" {
        init();
    }

    else if query == "import" {
        if authenticate() {
            let path = &args[2].strip_prefix("--path=").expect("Invalid argument").to_string();
            import(path);
        }
        else {
            println!("Invalid Master Password!");
        }
    }

}