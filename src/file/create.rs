// use std::collections::HashMap;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Seek, SeekFrom, Write};

pub fn create() -> std::io::Result<()> {
    let folder = "files";
    let wal_path = format!("{}/wal", folder);
    let main_path = format!("{}/db", folder);
    let checkpoint_path = format!("{}/checkpoint", folder);
    let index_path = format!("{}/index", folder);

    // создаем пример map
    // let mut map = HashMap::new();
    // map.insert("ключ1", "значение1");
    // map.insert("ключ2", "значение2");

    // сериализация
    let mut data = b"data";
    let len = data.len();

    create_dir_all(folder)?;

    // открываем основной файл
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .write(true)
        .open(&main_path)?;

    let main_offset = file.seek(SeekFrom::End(0))?;
    let main_offset_u64 = main_offset as u64;
    let len_u64 = len as u64;

    // --- записываем WAL: offset + len + data ---
    let mut wal = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&wal_path)?;
    wal.write_all(&main_offset_u64.to_le_bytes())?;
    wal.write_all(&len_u64.to_le_bytes())?;
    wal.write_all(data)?;
    wal.sync_all()?; // flush WAL

    // --- записываем основной файл:  data + len ---
    file.write_all(data)?;
    file.write_all(&len_u64.to_le_bytes())?;
    file.sync_all()?;

    // --- создаем checkpoint ---
    let mut checkpoint = File::create(&checkpoint_path)?;
    checkpoint.write_all(&main_offset_u64.to_le_bytes())?;
    checkpoint.write_all(&len_u64.to_le_bytes())?;
    checkpoint.sync_all()?;

    // --- добавляем в индекс ---
    let mut index = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&index_path)?;
    index.write_all(&main_offset_u64.to_le_bytes())?;
    index.write_all(&len_u64.to_le_bytes())?;
    index.sync_all()?;

    Ok(())
}
