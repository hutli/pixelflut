use pixelflut::alphabet_bitarr::BitMapAlphabet;
use pixelflut::packet::*;
use std::net::TcpStream;
use std::time::Instant;

const SERVER_IP: &str = "127.0.0.1:1337";
//const SERVER_IP: &str = "127.0.0.1:3333";
//const SERVER_IP: &str = "192.168.0.96:1337";
//const SERVER_IP:&str = "192.168.0.161:1337";

const RUNS: u32 = 10;

// https://i0.wp.com/luckyresistor.me/wp-content/uploads/2015/09/font1.png?ssl=1

fn connect() -> TcpStream {
    let mut stream = TcpStream::connect(SERVER_IP).expect("Failed to connect");
    println!("Successfully connected to server {}", SERVER_IP);
    return stream;
}

fn send_data(to_send: String) {
    let mut stream = connect();
    let mut packet_to_build = Packet::<BitMapAlphabet>::new(8);
    for _ in 0..RUNS {
        //loop {
        let now = Instant::now();
        packet_to_build.add_string(&to_send);
        let build_took = now.elapsed();
        // Send and reset
        let _ = packet_to_build.write(&mut stream);
        packet_to_build.reset();
        let total_took = now.elapsed();
        println!(
            "Packet build took {}μs out of {}μs ({:.2}%)",
            build_took.as_micros(),
            now.elapsed().as_micros(),
            (build_took.as_nanos() as f64 / total_took.as_nanos() as f64) * 100f64
        );
    }
}

fn send_noise(start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
    let mut stream = connect();
    let mut packet_to_build = Packet::<BitMapAlphabet>::new(8);
    packet_to_build.add_noise(start_x, start_y, end_x, end_y);
    for _ in 0..RUNS {
        let now = Instant::now();
        packet_to_build.recalculate_noise();
        let build_took = now.elapsed();
        // Send and reset
        let _ = packet_to_build.write(&mut stream);
        let total_took = now.elapsed();
        println!(
            "Packet build took {}μs out of {}μs ({:.2}%)",
            build_took.as_micros(),
            now.elapsed().as_micros(),
            (build_took.as_nanos() as f64 / total_took.as_nanos() as f64) * 100f64
        );
    }
}

fn create_and_consume(x: u32, y: u32, delay_ms: u128, chunks: u32) {
    let mut stream = connect();
    let mut packet_to_build = Packet::<BitMapAlphabet>::new(8);
    for i in 0..chunks {
        packet_to_build.add_area(i * x, 0, i * x + x, y, 0x123456);
    }
    let mut last_consume = Instant::now();
    let size = x * y;
    let mut i = chunks;
    while i > 0 {
        if last_consume.elapsed().as_millis() >= delay_ms {
            //println!("Extending with {}px", size);
            packet_to_build.extend_noise(size);
            last_consume = Instant::now();
            i -= 1;
        }
        packet_to_build.recalculate_noise();
        let _ = packet_to_build.write(&mut stream);
    }
}

fn main() {
    let input_arg = std::env::args().nth(1).expect("no input given");

    // send_data(input_arg);
    // send_noise(0, 0, 500, 500);
    create_and_consume(2, 50, 50, 2000);

    println!("Terminated.");
}
