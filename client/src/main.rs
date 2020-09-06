use pixelflut::packet::*;
use std::net::TcpStream;
use std::time::Instant;

const SERVER_IP: &str = "127.0.0.1:1337";
//const SERVER_IP: &str = "192.168.0.96:1337";
//const SERVER_IP:&str = "192.168.0.161:1337";

// https://i0.wp.com/luckyresistor.me/wp-content/uploads/2015/09/font1.png?ssl=1

fn send_data(to_send: String) {
    let mut stream = TcpStream::connect(SERVER_IP).expect("Failed to connect");
    println!("Successfully connected to server {}", SERVER_IP);
    let mut packet_to_build = Packet::new(8);
    loop {
        let now = Instant::now();
        packet_to_build.add_string(&to_send);
        let build_took = now.elapsed();
        // Send and reset
        let _ = packet_to_build.write(&mut stream);
        packet_to_build.reset();
        println!(
            "Build took {}μs out of {}μs",
            build_took.as_micros(),
            now.elapsed().as_micros()
        );
    }
}

fn main() {
    let input_arg = std::env::args().nth(1).expect("no input given");

    send_data(input_arg);

    println!("Terminated.");
}
