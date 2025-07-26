use std::io::{self, Write};

pub fn update_input(buf: &mut String, mes: &str) -> Result<usize, io::Error> {
    print!("{mes}");
    io::stdout().flush()?;

    buf.clear();
    let bytes_read = io::stdin().read_line(buf)?;

    let len = buf.trim_end().len();
    buf.truncate(len);
    Ok(bytes_read)
}

pub fn get_input(mes: &str) -> Result<String, io::Error> {
    let mut buf = String::new();
    update_input(&mut buf, mes)?;
    Ok(buf)
}
