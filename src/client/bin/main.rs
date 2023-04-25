use std::{net::TcpStream, io::{Write, Read}};
use classes::{Props, BMI, BUFFER_SIZE};

fn send_request(props: &Props) -> Result<BMI, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("localhost:8080")?;

    stream.write_all(serde_json::to_string(&props)?.as_bytes())?;
    stream.flush()?;
    println!("Sent data");

    stream.shutdown(std::net::Shutdown::Write)?;
    println!("Shutted down the write stream");

    let mut s = String::with_capacity(BUFFER_SIZE);
    stream.read_to_string(&mut s)?;
    let bmi = serde_json::from_str::<BMI>(&s)?;

    Ok(bmi)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut props = Props::new(0.1, 0f64);

    loop {
        props.height += 0.1;
        props.weight += 10f64;
        println!("Props: {:?}", props);
    
        let bmi = send_request(&props)?;
        println!("BMI: {:?}", bmi);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
