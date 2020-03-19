#![no_std]
use core::fmt;

extern crate alloc;

use alloc::{vec, vec::Vec, string::String};

pub mod prelude {
    pub use super::BinRead;
}

pub use binread;
pub use binread::{io, BinRead};

use binread::NullString;

#[derive(BinRead, Debug, Clone)]
pub struct TikFile {
    pub sig_type: SignatureType,
    
    #[br(pad_after("sig_type.padding()"))]
    #[br(count("sig_type.size()"))]
    pub signature: Vec<u8>,

    pub ticket_data: TicketData,
}



#[derive(BinRead, Debug, Clone)]
#[br(little)]
pub struct TicketData {
    #[br(map("|s: NullString| s.into_string()"))]
    #[br(pad_size_to(0x40))]
    pub issuer: String,

    #[br(pad_size_to(0x100))]
    pub title_key: [u8; 0x10],

    always_two: u8,
    pub title_key_type: u8,
    unk: [u8; 3],
    pub mkey_generation: u8,

    unk2: [u8; 0xA],
    pub ticket_id: u64,
    pub device_id: u64,
    pub rights_id: [u8; 0x10],

    pub account_id: u32
}

#[derive(BinRead, Debug, Clone, Copy)]
pub enum SignatureType {
    #[br(magic(0x01_0000u32))] Rsa4096Sha1,
    #[br(magic(0x01_0001u32))] Rsa2048Sha1,
    #[br(magic(0x01_0002u32))] EcdsaSha1,
    #[br(magic(0x01_0003u32))] Rsa4096Sha256,
    #[br(magic(0x01_0004u32))] Rsa2048Sha256,
    #[br(magic(0x01_0005u32))] EcdsaSha256,
}

use SignatureType::*;

impl fmt::Display for SignatureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rsa4096Sha1 => "RSA-4096 SHA1",
                Rsa4096Sha256 => "RSA-4096 SHA256",
                Rsa2048Sha1 => "RSA-2048 SHA1",
                Rsa2048Sha256 => "RSA-2048 SHA256",
                EcdsaSha1 => "ECDSA SHA1",
                EcdsaSha256 => "ECDSA SHA256",
            }
        )
    }
}

impl SignatureType {
    fn size(&self) -> usize {
        match self {
            Rsa4096Sha1 | Rsa4096Sha256 => 0x200,
            Rsa2048Sha1 | Rsa2048Sha256 => 0x100,
            EcdsaSha1 | EcdsaSha256 => 0x3C,
        }
    }
    
    fn padding(&self) -> usize {
        match self {
            EcdsaSha1 | EcdsaSha256 => 0x40,
            _ => 0x3C,
        }
    }
}
