use std::collections::HashMap;

const A: char = 'A';
const C: char = 'C';
const G: char = 'G';
const T: char = 'T';
const NUCLEOTIDES: [char; 4] = [A, C, G, T];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    dna.chars().try_fold(0, |acc, c| {
        if !NUCLEOTIDES.contains(&c) {
            Err(c)
        } else if nucleotide == c {
            Ok(acc + 1)
        } else {
            Ok(acc)
        }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut acc = HashMap::with_capacity(4);

    NUCLEOTIDES.into_iter().for_each(|c| {
        acc.insert(c, 0);
        ()
    });

    dna.chars().try_fold(acc, |mut acc, c| {
        if !NUCLEOTIDES.contains(&c) {
            Err(c)
        } else {
            *acc.entry(c).or_insert(0) += 1;
            Ok(acc)
        }
    })
}
