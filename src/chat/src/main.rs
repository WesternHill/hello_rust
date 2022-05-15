use std::env;
use std::net::{TcpListener,TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::thread;
use std::io;


fn server(ip: &String, port:u32) {
    println!("server ip:{} port:{}",ip,port);

    // Set IP and Port
    let addr = ip.to_string() + ":" + &port.to_string();

    // Configure TCP socket
    let listener = TcpListener::bind(addr).expect("Couldn't bind");

    // Listen and wait for incoming connection
    for stream in listener.incoming() {
        // Streamが返すResult型（Ok/Err）に応じて処理
        match stream
        {
            Err(e) => { eprintln!("listern.incoming failed due to {}",e); }// When stream is error.
            Ok(mut stream) => {
                thread::spawn( // 別スレッドを発生させて引数の関数を実行する (spawn:発生させる)
                    move || { // moveで、ラムダ（クロージャ）関数に渡す引数の所有権譲渡を強制する
                        loop{
                            let mut buf: [u8; 512] = [0; 512]; // 512 byte buffer formatted with 0
                            let byte_read = stream.read(&mut buf).expect("Faild to read_exact"); //&mutをつけて、msgを参照かつ編集可能にする
                            let rcvd = from_utf8(&buf).expect("Faild to read_exact"); // uint8型を文字列として解釈
                            println!("rcvd:{}",rcvd);                    
                        }
                    }
                );
            }
        }
    }
}

fn client(ip: &String, port:u32) {
    println!("client ip:{} port:{}",ip,port);
    let addr = ip.to_string() + ":" + &port.to_string();

    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            loop {
                // Read user key input
                let mut user_keyinput = String::new();
                io::stdin()
                    .read_line(&mut user_keyinput)
                    .expect("Failed to readline");
    
                let input = &user_keyinput;
    
                match stream.write(input.as_bytes()) { 
                  Ok(rslt) => { println!("Sent {} {}",rslt,input); } 
                  Err(e) => { panic!("{}",e); }
                } // ?をつけることで、write内部でpanicが起こった場合はErrを返してくれる
            }
        }
        Err(e) => {
            eprintln!("Connection failed by {}",e);
        }
    }
    
    // if let Ok(mut stream) = TcpStream::connect(addr) {
    //     loop {
    //         // Read user key input
    //         let mut user_keyinput = String::new();
    //         io::stdin()
    //             .read_line(&mut user_keyinput)
    //             .expect("Failed to readline");

    //         let input = &user_keyinput;

    //         match stream.write(input.as_bytes()) { 
    //           Ok(rslt) => { println!("Sent {} {}",rslt,input); } 
    //           Err(e) => { panic!("{}",e); }
    //         } // ?をつけることで、write内部でpanicが起こった場合はErrを返してくれる
    //     }
    //     // Ok(()); //これがないと”何も返さない関数”扱いされて、"?"オペランドでエラーになる
    // }
}

fn main() {
    // Get command line arguments. Argument contains mode([c]lient/[s]ervers), ip, port
    let args: Vec<String> = env::args().collect();

    /* Get commandline arguments */
    let mode = &args[1];// cnvert uint8
    let ip = &args[2]; // just trimmed unnecessary charactor like ¥n
    let port :u32 = (&args[3]).trim().parse().expect("FAIL: 3rd param proc"); // convert to uint32

    println!("mode:{} ip:{} port:{}",mode,ip,port);

    if "s" == mode {
        server(ip, port);     // if server mode, app will wait for client connections
    }else if "c" == mode {
        client(ip, port);    // if client mode, app will connect to servers
    }


}
