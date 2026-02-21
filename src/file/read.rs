use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};

pub fn read() -> Result<()> {
    let mut file = File::open("files/index").unwrap();

    let offset = 0;

    let b_offset: u64 = offset * 16;
    file.seek(SeekFrom::Start(b_offset))?;

    let mut buffer = [0u8; 16];
    file.read_exact(&mut buffer)?;

    let dboffset = u64::from_le_bytes(buffer[0..8].try_into().unwrap());
    let dbvalue = u64::from_le_bytes(buffer[8..16].try_into().unwrap());

    println!("{} - {}", dboffset, dbvalue);

    let mut db = File::open("files/db").unwrap();

    db.seek(SeekFrom::Start(dboffset))?;

    let mut buffer = vec![0u8; dbvalue as usize];
    db.read_exact(&mut buffer)?;

    // десериализация
    let deserialized = "bytes";

    println!("{:?}", deserialized);

    Ok(())
}
