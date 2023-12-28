use crate::messages::{Header, Message};
use std::io::{prelude::*, ErrorKind};
use std::mem;
use std::net::TcpStream;

fn create_header(data: &Vec<u8>) -> Vec<u8> {
    let header = Header::new(data.len());
    let header: Vec<u8> = match bincode::serialize(&header) {
        Ok(h) => h,
        Err(e) => panic!("failed to serialize header: {:?}", e),
    };

    // Ensure that bincode does not change header size when serializing it
    assert_eq!(mem::size_of::<Header>(), header.len());

    header
}

pub fn send(stream: &mut TcpStream, msg: Message) {
    let mut data: Vec<u8> = match bincode::serialize(&msg) {
        Ok(buf) => buf,
        Err(e) => panic!("failed to serialize message: {:?}", e),
    };

    let mut header = create_header(&data);

    let mut buffer: Vec<u8> = Vec::new();
    buffer.append(&mut header);
    buffer.append(&mut data);

    let mut pos: usize = 0;
    while pos < buffer.len() {
        let bytes_written: usize = match stream.write(&buffer[pos..]) {
            Ok(bytes) => match bytes {
                0 => panic!("stream is no longer able to accept bytes"),
                b => b,
            },
            Err(e) => match e.kind() {
                ErrorKind::Interrupted => 0,
                _ => panic!("failed to send bytes: {:?}", e),
            },
        };
        println!("sent {} bytes", bytes_written);
        pos += bytes_written;
    }
}

fn read_bytes(stream: &mut TcpStream, n_bytes: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0u8; n_bytes];

    let mut pos: usize = 0;
    while pos < n_bytes {
        let bytes_read: usize = match stream.read(&mut buffer[pos..]) {
            Ok(bytes) => match bytes {
                0 => panic!("connection has been shut down"),
                b => b,
            },
            Err(e) => match e.kind() {
                ErrorKind::Interrupted => 0,
                _ => panic!("failed to receive bytes: {:?}", e),
            },
        };
        println!("received {} bytes", bytes_read);
        pos += bytes_read;
    }

    buffer
}

fn read_header(stream: &mut TcpStream) -> Header {
    let header_len = mem::size_of::<Header>();
    let header: Vec<u8> = read_bytes(stream, header_len);

    let header: Header = match bincode::deserialize(&header) {
        Ok(h) => h,
        Err(e) => panic!("failed to deserialize header: {:?}", e),
    };

    header
}

pub fn receive(stream: &mut TcpStream) -> Message {
    let header = read_header(stream);
    let payload_len: usize = header.payload_len as usize;

    let data: Vec<u8> = read_bytes(stream, payload_len);
    let data: Message = match bincode::deserialize(&data) {
        Ok(d) => d,
        Err(e) => panic!("failed to deserialize message: {:?}", e),
    };

    data
}
