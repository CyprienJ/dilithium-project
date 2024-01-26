extern crate pqc_dilithium;
use pqc_dilithium::*;
use std::fs::File;
use std::io::Write;
use std::fs::read;



fn encode(path_file: &str, keys: &Keypair){
   
    //reading the message file
    let msg = match read(path_file){
        Err(why) => panic!("couldn't read key file: {}" , why),
        Ok(msg) => msg,        
    };

    let sig: [u8; 3293] = keys.sign(&msg);

    //writing the signature file
    let path = "signature.txt";
    let mut output: File = match File::create(path){
        Err(why) => panic!("couldn't create key file: {}" , why),
        Ok(file) => file,
    };

    for i in 0..3293{
        match write!(output, "{}\n",sig[i]){
            Err(why) => panic!("couldn't write in key file: {}" , why),
            Ok(_) => print!(""),
        };
    }

}


fn main() {
    //generating the key
    let keys: Keypair = Keypair::generate();
    let public_key = keys.public;

    //encoding the file
    encode("input.txt", &keys);

    //writing the key
    let path = "key.txt";
    let mut output: File = match File::create(path){
        Err(why) => panic!("couldn't create key file: {}" , why),
        Ok(file) => file,
    };

    for i in 0..1952{
        match write!(output, "{}\n",public_key[i]){
            Err(why) => panic!("couldn't write in key file: {}" , why),
            Ok(_) => print!(""),
        };
    }

    //function to check localy if the code works
    if false{
        let msg = match read("input.txt"){
            Err(why) => panic!("couldn't read key file: {}" , why),
            Ok(msg) => msg,        
        };

        let sig: [u8; 3293] = keys.sign(&msg);
        
        let soluce = verify(&sig, &msg, &public_key);

        if soluce.is_ok() {
            println!("document conforme");
        }else{
            println!("document non conforme");
        }
    }

}



