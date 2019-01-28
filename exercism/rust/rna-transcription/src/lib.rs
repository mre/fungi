#[derive(Debug, PartialEq)]
pub struct DNA {
    sequence: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    sequence: String,
}

const NUCLEOTIDES_DNA: &str = "ACGT";
const NUCLEOTIDES_RNA: &str = "ACGU";

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, nucleotide) in dna.chars().enumerate() {
            if !NUCLEOTIDES_DNA.contains(nucleotide) {
                Err(i)?;
            }
        }
        Ok(DNA {
            sequence: String::from(dna),
        })
    }

    pub fn to_rna(self) -> RNA {
        RNA::new(
            &(self
                .sequence
                .chars()
                .map(|n| match n {
                    'A' => 'U',
                    'C' => 'G',
                    'G' => 'C',
                    'T' => 'A',
                    _ => n,
                })
                .collect::<String>()),
        )
        .unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, nucleotide) in rna.chars().enumerate() {
            if !NUCLEOTIDES_RNA.contains(nucleotide) {
                Err(i)?;
            }
        }
        Ok(RNA {
            sequence: String::from(rna),
        })
    }
}
