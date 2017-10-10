use std::convert::Into;

type DnaResult<T> = Result<T, ()>;

#[derive(Debug, Clone, PartialEq)]
pub struct RibonucleicAcid {
    strand: String,
}

impl RibonucleicAcid {
    pub fn new<T: Into<String>>(strand: T) -> RibonucleicAcid {
        RibonucleicAcid { strand: strand.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeoxyribonucleicAcid {
    strand: String,
}

impl DeoxyribonucleicAcid {
    pub fn new<T: Into<String>>(strand: T) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { strand: strand.into() }
    }

    pub fn to_rna(self) -> DnaResult<RibonucleicAcid> {
        let rna_strand = self.strand.chars().filter_map(|ch| match ch {
            'G' => Some('C'),
            'C' => Some('G'),
            'T' => Some('A'),
            'A' => Some('U'),
            _   => None,
        }).collect::<String>();
        match rna_strand.len() == self.strand.len() {
            true => Ok(RibonucleicAcid { strand: rna_strand }),
            false => Err(()),
        }
    }
}
