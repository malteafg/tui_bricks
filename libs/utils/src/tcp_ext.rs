use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TcpError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serialize(postcard::Error),
    #[error("Deserialization error: {0}")]
    Deserialize(postcard::Error),
}

pub trait TcpExt {
    fn send<T: Serialize>(&mut self, value: &T) -> Result<(), TcpError>;

    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, TcpError>;

    fn send_and_receive<T: Serialize, S: for<'a> Deserialize<'a>>(
        &mut self,
        value: &T,
    ) -> Result<S, TcpError> {
        self.send(value)?;
        self.receive()
    }
}

impl TcpExt for TcpStream {
    fn send<T: Serialize>(&mut self, value: &T) -> Result<(), TcpError> {
        let data = postcard::to_stdvec(value).map_err(|e| TcpError::Serialize(e))?;

        let len = (data.len() as u32).to_le_bytes();
        self.write_all(&len)?;
        self.write_all(&data)?;
        Ok(())
    }

    fn receive<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, TcpError> {
        let mut len_buf = [0u8; 4];
        self.read_exact(&mut len_buf)?;
        let msg_len = u32::from_le_bytes(len_buf) as usize;

        let mut buf = vec![0u8; msg_len];
        self.read_exact(&mut buf)?;

        let data = postcard::from_bytes(&buf).map_err(|e| TcpError::Deserialize(e))?;

        Ok(data)
    }
}

// use rkyv::{
//     Archive, Deserialize, Serialize,
//     api::high::{HighSerializer, HighValidator},
//     bytecheck::CheckBytes,
//     de::Pool,
//     rancor::Strategy,
//     ser::allocator::ArenaHandle,
//     util::AlignedVec,
// };

// pub trait TcpExt {
//     fn send<T>(&mut self, value: &T) -> Result<(), TcpError>
//     where
//         T: for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, rkyv::rancor::Error>>;

//     fn receive<T>(&mut self) -> Result<T, TcpError>
//     where
//         T: Archive,
//         T::Archived: for<'a> CheckBytes<HighValidator<'a, rkyv::rancor::Error>>
//             + Deserialize<T, Strategy<Pool, rkyv::rancor::Error>>;

//     fn send_and_receive<T, S>(&mut self, value: &T) -> Result<S, TcpError>
//     where
//         T: for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, rkyv::rancor::Error>>,
//         S: Archive,
//         S::Archived: for<'a> CheckBytes<HighValidator<'a, rkyv::rancor::Error>>
//             + Deserialize<S, Strategy<Pool, rkyv::rancor::Error>>,
//     {
//         self.send(value)?;
//         self.receive()
//     }
// }

// impl TcpExt for TcpStream {
//     fn send<T>(&mut self, value: &T) -> Result<(), TcpError>
//     where
//         T: for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, rkyv::rancor::Error>>,
//     {
//         let data = rkyv::to_bytes(value).map_err(|_| TcpError::Serialize)?;

//         let len = (data.len() as u32).to_le_bytes();
//         self.write_all(&len)?;
//         self.write_all(&data)?;
//         Ok(())
//     }

//     fn receive<T>(&mut self) -> Result<T, TcpError>
//     where
//         T: Archive,
//         T::Archived: for<'a> CheckBytes<HighValidator<'a, rkyv::rancor::Error>>
//             + Deserialize<T, Strategy<Pool, rkyv::rancor::Error>>,
//     {
//         let mut len_buf = [0u8; 4];
//         self.read_exact(&mut len_buf)?;
//         let msg_len = u32::from_le_bytes(len_buf) as usize;

//         let mut buf = vec![0u8; msg_len];
//         self.read_exact(&mut buf)?;

//         let data = rkyv::from_bytes(&buf).map_err(|_| TcpError::Deserialize)?;

//         Ok(data)
//     }
// }
