use std::{
    io::{self, Error, Write},
    str::FromStr,
};
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