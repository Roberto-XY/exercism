#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines = input.split('\n').collect::<Vec<_>>();

    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }

    if let Some(column_count) = lines.iter().find(|line| line.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(column_count.len()));
    }

    let col_len = lines.len();
    let row_len = lines.first().unwrap().len();

    let mut acc = vec![];

    for x in (0..col_len).step_by(4) {
        for y in (0..row_len).step_by(3) {
            let string_region = lines[x..x + 4]
                .iter()
                .map(|s| s[y..y + 3].to_string())
                .collect::<Vec<_>>();
            acc.push(string_region);
        }
    }

    let res = acc
        .chunks_exact(row_len / 3)
        .map(|rows| {
            rows.iter()
                .map(|row| {
                    ocr_to_digit(
                        row.iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .as_slice(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .map(|row| row.join(""))
        .collect::<Vec<_>>()
        .join(",");

    Ok(res)
}

fn ocr_to_digit(ocr: &[&str]) -> &'static str {
    match ocr {
        [" _ ", "| |", "|_|", "   "] => "0",
        ["   ", "  |", "  |", "   "] => "1",
        [" _ ", " _|", "|_ ", "   "] => "2",
        [" _ ", " _|", " _|", "   "] => "3",
        ["   ", "|_|", "  |", "   "] => "4",
        [" _ ", "|_ ", " _|", "   "] => "5",
        [" _ ", "|_ ", "|_|", "   "] => "6",
        [" _ ", "  |", "  |", "   "] => "7",
        [" _ ", "|_|", "|_|", "   "] => "8",
        [" _ ", "|_|", " _|", "   "] => "9",
        _ => "?",
    }
}
