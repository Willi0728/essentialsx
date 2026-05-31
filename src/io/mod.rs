pub mod repl;

#[cfg(feature = "io")]
use std::{
    io::{self, Error, Write, BufRead},
    str::FromStr,
};

#[cfg(feature = "io")]
pub fn input<T: FromStr>(prompt: &str) -> io::Result<T>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    print!("{prompt}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.pop();
    if input.ends_with('\r') {
        input.pop();
    }
    input.parse::<T>().map_err(Error::other)
}

#[cfg(feature = "io")]
pub fn input_bytes_until_char(prompt: &str, end: u8) -> io::Result<Vec<u8>> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut bytes = vec![];
    io::stdin().lock().read_until(end, &mut bytes)?;
    Ok(bytes)
}