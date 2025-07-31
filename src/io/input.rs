use std::io::{self, Write};

/// Update an existing `String` with new input.
///
/// Prints a message, then clears and repopulates a given `String` with a line from `stdin`.
/// This will additionally trim any trailing whitespace from `buf`.
///
/// # Parameters
/// `buf`: The `String` buffer to be updated.
/// `mes`: The message to be printed.
///
/// # Returns
/// A `Result` which is either the number of bytes read from `stdin` (including the trimmed
/// whitespace) or an `io::Error`.
pub fn update_input(buf: &mut String, mes: &str) -> Result<usize, io::Error> {
    print!("{mes}");
    io::stdout().flush()?;

    buf.clear();
    let bytes_read = io::stdin().read_line(buf)?;

    let len = buf.trim_end().len();
    buf.truncate(len);
    Ok(bytes_read)
}

/// Create a new `String` from `stdin`.
///
/// Prints a message, then allocates and populates a `String` with a line from `stdin`. Any
/// trailing whitespace will be trimmed from the `String` before returning.
///
/// # Parameters
/// `mes`: The message to be printed.
///
/// # Returns
/// A `Result` which is either the new `String` or an `io::Error`.
pub fn get_input(mes: &str) -> Result<String, io::Error> {
    let mut buf = String::new();
    update_input(&mut buf, mes)?;
    Ok(buf)
}
