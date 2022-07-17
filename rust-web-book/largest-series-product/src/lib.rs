#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let v_alphanumeric = string_digits.matches(char::is_alphabetic).collect::<Vec<&str>>();
    
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    else if span == 0 {
        return Ok(1u64);
    }
    else if v_alphanumeric.len() > 0 {
        return Err(Error::InvalidDigit(
            v_alphanumeric
            .first().unwrap()
            .to_owned()
            .to_string()
            .pop()
            .unwrap()
        ));
    }

    let mut digits: Vec<u64> = string_digits
        .split("")
        .map(|digit| digit.parse())
        .filter(|digit| match digit {
            Ok(_) => true,
            Err(_) => false,
        })
        .map(|d| d.unwrap())
        .collect();
    digits.reverse();
    Ok(window_values(digits, span)
        .first()
        .unwrap()
        .to_owned()
    )
}

pub fn window_values(digits: Vec<u64>, span: usize) -> Vec<u64>{
    let mut window_values = digits
        .windows(span)
        .map(|w| w.to_vec())
        .map(|v| v.iter().fold(1, |acc, i| acc * i))
        .collect::<Vec<u64>>();
    window_values.sort();
    window_values.reverse();
    window_values
} 