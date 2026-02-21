use std::fs::OpenOptions;
use std::io::{BufReader, Read, Result, Seek, SeekFrom, Write};

const MAX_RECORD_SIZE: usize = 16 * 1024 * 1024;

pub fn recovery() -> Result<()> {
    let folder = "files";
    let wal_path = format!("{}/wal", folder);

    let main_path = format!("{}/db", folder);

    let mut main_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&main_path)
        .unwrap();

    let wal_file = OpenOptions::new().read(true).open(&wal_path).unwrap();
    let mut wal_reader = BufReader::new(wal_file);

    loop {
        let mut offset_buf = [0u8; 8];
        let mut len_buf = [0u8; 8];

        if wal_reader.read_exact(&mut offset_buf).is_err() {
            break;
        }

        if wal_reader.read_exact(&mut len_buf).is_err() {
            break;
        }

        let offset = u64::from_le_bytes(offset_buf);
        let len = u64::from_le_bytes(len_buf);

        if len == 0 || len as usize > MAX_RECORD_SIZE {
            eprintln!("WAL повреждён: некорректная длина {}", len);
            break;
        }

        // целая ли запись
        let mut data = vec![0u8; len as usize];
        if wal_reader.read_exact(&mut data).is_err() {
            break;
        }

        // нужна ли запись
        let file_len = main_file.seek(SeekFrom::End(0)).unwrap();
        if file_len >= offset + len {
            continue;
        }

        // пишем с offset
        main_file.seek(SeekFrom::Start(offset)).unwrap();
        main_file.write_all(&data).unwrap();
        main_file.write_all(&len.to_le_bytes()).unwrap();
        main_file.flush().unwrap();
        main_file.sync_all().unwrap();
    }

    // чистим WAL
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&wal_path)
        .unwrap();

    Ok(())
}
