use chrono::{Local, Timelike};
use clap::Parser;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::net::UdpSocket;
use std::{thread, time};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Receiver address
    #[arg(short, long, default_value_t = String::from("127.0.0.1:9000"))]
    receiver: String,

    /// Sender address
    #[arg(long, default_value_t = String::from("127.0.0.1:9002"))]
    sender: String,
}

fn main() {
    let arguments = Arguments::parse();
    let socket = UdpSocket::bind(&arguments.sender).unwrap();
    let sleep_duration = time::Duration::from_millis(100);
    loop {
        let time = Local::now();
        let second = time.num_seconds_from_midnight() as f32;
        let minute = second / 60.0;
        let hour = minute / 60.0;

        let second_message = OscPacket::Message(OscMessage {
            addr: String::from("/avatar/parameters/TimeSecond"),
            args: vec![OscType::Float(second % 60.0 / 60.0)],
        });
        let minute_message = OscPacket::Message(OscMessage {
            addr: String::from("/avatar/parameters/TimeMinute"),
            args: vec![OscType::Float(minute % 60.0 / 60.0)],
        });
        let hour_message = OscPacket::Message(OscMessage {
            addr: String::from("/avatar/parameters/TimeHour"),
            args: vec![OscType::Float(hour % 12.0 / 12.0)],
        });

        let second_buffer = encoder::encode(&second_message).unwrap();
        let minute_buffer = encoder::encode(&minute_message).unwrap();
        let hour_buffer = encoder::encode(&hour_message).unwrap();

        socket.send_to(&second_buffer, &arguments.receiver).unwrap();
        socket.send_to(&minute_buffer, &arguments.receiver).unwrap();
        socket.send_to(&hour_buffer, &arguments.receiver).unwrap();

        thread::sleep(sleep_duration);
    }
}
