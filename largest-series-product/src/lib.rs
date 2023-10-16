#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() { return Err(Error::SpanTooLong); }
    if let Some(ch) = string_digits.chars().find(|c| !c.is_ascii_digit()) {
        return Err(Error::InvalidDigit(ch));
    }
    let di = string_digits.chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>();
    Ok(di.as_slice()
        .windows(span)
        .map(|arr| arr.iter().product::<u64>())
        .into_iter()
        .max()
        .unwrap()
    )
}
