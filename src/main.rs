use prettytable::{table, row, cell, format::{FormatBuilder, LinePosition, LineSeparator}};
use nxtik::{TikFile, prelude::*};
use structopt::StructOpt;
use std::{fs::File, path::PathBuf};
use std::fmt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(short, long)]
    signature: bool,

    #[structopt(short, long, conflicts_with="signature")]
    key_only: bool,

    #[structopt(short, long, conflicts_with="signature", conflicts_with="key_only")]
    rid_only: bool,

    file: PathBuf
}

macro_rules! hex {
    ($x:expr) => {
        format!("0x{:X}", $x)
    }
}

fn main() -> nxtik::binread::BinResult<()> {
    let args = Args::from_args();

    let mut file = File::open(args.file)?;

    let tik = TikFile::read(&mut file)?;

    if args.key_only {
        println!("{}", Hex(&tik.ticket_data.title_key[..]));
    } else if args.rid_only {
        println!("{}", Hex(&tik.ticket_data.rights_id[..]));
    } else if args.signature {
        println!("{}", Hex(&tik.signature[..]));
    } else {
        // Print out table of all properties
        let mut table = table!(
            ["Signature Type:", tik.sig_type],
            ["Signature:", CutOff(Hex(&tik.signature[..0xE]))],
            ["Issuer:", tik.ticket_data.issuer],
            [bFy=>"Title Key:", Hex(&tik.ticket_data.title_key[..])],
            ["Title Key Type:", tik.ticket_data.title_key_type],
            ["Key Generation:", tik.ticket_data.mkey_generation],
            ["Ticket ID:", hex!(tik.ticket_data.ticket_id)],
            ["Device ID:", hex!(tik.ticket_data.device_id)],
            [bFy=>"Rights ID:", Hex(&tik.ticket_data.rights_id[..])],
            ["Account ID:", hex!(tik.ticket_data.account_id)]
        );
        table.set_format(
            FormatBuilder::new()
                .column_separator('\t')
                .borders(' ')
                .separators(&[], LineSeparator::new('-', ' ', ' ', ' '))
                .build()
        );
        table.printstd();
    }

    Ok(())
}

#[repr(transparent)]
struct Hex<'a>(&'a [u8]);

#[repr(transparent)]
struct CutOff<D: fmt::Display + Sized>(D);

impl<D: fmt::Display + Sized> fmt::Display for CutOff<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}...", self.0)
    }
}

impl<'a> fmt::Display for Hex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}
