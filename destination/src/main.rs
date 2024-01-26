extern crate pqc_dilithium;
use pqc_dilithium::*;
use std::fs::{read, read_to_string};


fn check(file_path: &str, sig_path: &str, key : &[u8]){

    //reading the signature
    let mut result = Vec::new();
    for line in read_to_string(sig_path).unwrap().lines() {
        match line.parse::<u8>(){
            Err(_) => () ,
            Ok( value) => result.push(value)
        }
    }
    let sig = result;

    //reading the message to verify
    let msg = match read(file_path){
        Err(why) => panic!("couldn't read key file: {}" , why),
        Ok(msg) => msg,        
    };


    //verify the message
    let sig_verify = verify(&sig, &msg, &key );

    //prompting the message
    if sig_verify.is_ok() {
        println!("document ok");
    }else{
        println!("document, key or signature not ok");
    }

}
fn read_key(path: &str)-> Vec<u8>{
    //reading the key file
    let mut result = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        match line.parse::<u8>(){
            Err(_) => () ,
            Ok( value) => result.push(value)
        }
    }
    return result;
}


fn main() {

    //reading the key from file
    let key = read_key("key.txt");
    //verifying the signature
    check("input.txt", "signature.txt", &key);


}



