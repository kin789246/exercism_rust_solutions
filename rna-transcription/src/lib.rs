#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: String
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        //unimplemented!("Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide");
        for (i, c) in dna.chars().enumerate() {
            if c!='A' && c!='C' && c!='G' && c!='T' {
                return Err(i)
            }
        }
        Ok(Dna { nucleotides: dna.to_string() })
    }

    pub fn into_rna(self) -> Rna {
        //unimplemented!("Transform Dna {self:?} into corresponding Rna");
        let mut rna: String = String::new();
        self.nucleotides.chars().for_each(|c| {
            match c {
                'G' => rna += "C",
                'C' => rna += "G",
                'T' => rna += "A",
                _ => rna += "U",
            }
        });
        Rna { nucleotides: rna }
    }
}

impl<'a> Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        //unimplemented!("Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide");
        for (i, c) in rna.chars().enumerate() {
            if c!='A' && c!='C' && c!='G' && c!='U' {
                return Err(i)
            }
        }
        Ok(Rna { nucleotides: rna.to_string() })
    }
}
