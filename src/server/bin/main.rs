use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use classes::{Props, BMI};

fn handle_stream(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buff = String::new();
    stream.read_to_string(&mut buff)?;
    println!("Got buffer: {:?}", buff);

    let props = serde_json::from_str::<Props>(&buff)?;
    println!("Got props: {:?}", props);

    let bmi = serde_json::to_string(&BMI::get_bmi(&props))?;
    stream.write(bmi.as_bytes())?;
    println!("Sent response: {:?}", bmi);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("localhost:8080")?;
    
    for stream in listener.incoming() {
        println!("Accepted connection");
        match stream {
            Ok(stream) => handle_stream(stream)?,
            Err(err) => println!("Failed to accept stream: {:?}", err),
        }
    }

    Ok(())
}
