pub fn verse(n: u32) -> String {
    if n == 0 {
        String::from(
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
        )
    } else if n == 1 {
        String::from(
            "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
        )
    } else if n == 2 {
        String::from(
            "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n",
        )
    } else {
        format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n",
            n,
            n - 1
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut acc = (end..start + 1)
        .rev()
        .map(|i| verse(i))
        .fold(String::new(), |mut acc, verse| {
            acc.push_str(&verse);
            acc.push('\n');
            acc
        });
    acc.pop();
    acc
}
