use bincode::{Decode, Encode};

use std::io::{Read, Write};
use std::net::TcpStream;

pub trait TcpExt {
    fn send<Q: Encode>(&mut self, query: Q);
    fn receive<R: Decode<()>>(&mut self) -> R;

    fn send_and_receive<Q: Encode, R: Decode<()>>(&mut self, query: Q) -> R {
        self.send(query);
        self.receive()
    }
}

impl TcpExt for TcpStream {
    fn send<Q: Encode>(&mut self, query: Q) {
        let data = bincode::encode_to_vec(&query, bincode::config::standard()).unwrap();
        let len = (data.len() as u32).to_le_bytes();

        self.write_all(&len).unwrap();
        self.write_all(&data).unwrap();
    }

    fn receive<R: Decode<()>>(&mut self) -> R {
        let mut len_buf = [0u8; 4];
        self.read_exact(&mut len_buf).unwrap();
        let msg_len = u32::from_le_bytes(len_buf) as usize;

        let mut buf = vec![0u8; msg_len];
        self.read_exact(&mut buf).unwrap();

        let (response, _): (R, usize) =
            bincode::decode_from_slice(&buf, bincode::config::standard()).unwrap();

        response
    }
}
