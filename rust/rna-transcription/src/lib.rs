#[derive(Debug, PartialEq)]
pub enum DnaNucleotide {
    A,
    C,
    G,
    T,
}

#[derive(Debug, PartialEq)]
pub enum RnaNucleotide {
    A,
    C,
    G,
    U,
}

#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: Vec<DnaNucleotide>,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: Vec<RnaNucleotide>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucleotides = dna
            .chars()
            .enumerate()
            .map(|(idx, c)| match c {
                'A' => Ok(DnaNucleotide::A),
                'C' => Ok(DnaNucleotide::C),
                'G' => Ok(DnaNucleotide::G),
                'T' => Ok(DnaNucleotide::T),
                _ => Err(idx),
            })
            .try_fold(Vec::with_capacity(dna.len()), |mut acc, res| match res {
                Err(idx) => Err(idx),
                Ok(nucleotide) => {
                    acc.push(nucleotide);
                    Ok(acc)
                }
            })?;

        Ok(Dna { nucleotides })
    }

    pub fn into_rna(self) -> Rna {
        let nucleotides = self
            .nucleotides
            .iter()
            .map(|nucleotide| match nucleotide {
                DnaNucleotide::A => RnaNucleotide::U,
                DnaNucleotide::C => RnaNucleotide::G,
                DnaNucleotide::G => RnaNucleotide::C,
                DnaNucleotide::T => RnaNucleotide::A,
            })
            .collect::<Vec<_>>();

        Rna { nucleotides }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucleotides = rna
            .chars()
            .enumerate()
            .map(|(idx, c)| match c {
                'C' => Ok(RnaNucleotide::C),
                'G' => Ok(RnaNucleotide::G),
                'A' => Ok(RnaNucleotide::A),
                'U' => Ok(RnaNucleotide::U),
                _ => Err(idx),
            })
            .try_fold(Vec::with_capacity(rna.len()), |mut acc, res| match res {
                Err(idx) => Err(idx),
                Ok(nucleotide) => {
                    acc.push(nucleotide);
                    Ok(acc)
                }
            })?;

        Ok(Rna { nucleotides })
    }
}
