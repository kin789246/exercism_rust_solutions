use std::collections::HashMap;

#[warn(dead_code)]

#[derive(Debug)]
pub struct CodonsInfo<'a> {
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    // phantom: std::marker::PhantomData<&'a ()>,
    rna_map : HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.rna_map.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .map(|codon| self.name_for(codon.unwrap()))
            .take_while(|&codon| codon != Some("stop codon"))
            .collect()
//         let mut st = 0;
//         let codon_len = 3;
//         let mut ans: Vec<&'a str> = Vec::new();
//         while st < rna.len() {
//             if let Some(codon) = rna.get(st..st+codon_len) {
//                 if let Some(protein) = self.rna_map.get(codon) {
//                     match *protein {
//                         "stop codon" => break,
//                         _ => ans.push(*protein)
//                     }
//                 }
//                 st += codon_len;
//             }
//             else {
//                 return None;
//             }
//         }
// 
//         if ans.len() == 0 {
//             return None;
//         }
//         Some(ans)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        rna_map: pairs.iter().cloned().collect()
    }
    // let mut rna_map : HashMap<&'a str, &'a str> = HashMap::new();
    // for (codon, name) in pairs {
    //     rna_map.insert(codon, name);
    // }
    // CodonsInfo { rna_map: rna_map }
}


// Codon	            Protein
// -------------------  --------------
// AUG	                Methionine
// UUU, UUC	            Phenylalanine
// UUA, UUG	            Leucine
// UCU, UCC, UCA, UCG	Serine
// UAU, UAC	            Tyrosine
// UGU, UGC	            Cysteine
// UGG	                Tryptophan
// UAA, UAG, UGA	    STOP
//    let grouped = vec![
//         ("isoleucine", vec!["AUU", "AUC", "AUA"]),
//         ("valine", vec!["GUU", "GUC", "GUA", "GUG"]),
//         ("phenylalanine", vec!["UUU", "UUC"]),
//         ("methionine", vec!["AUG"]),
//         ("cysteine", vec!["UGU", "UGC"]),
//         ("alanine", vec!["GCU", "GCC", "GCA", "GCG"]),
//         ("glycine", vec!["GGU", "GGC", "GGA", "GGG"]),
//         ("proline", vec!["CCU", "CCC", "CCA", "CCG"]),
//         ("threonine", vec!["ACU", "ACC", "ACA", "ACG"]),
//         ("serine", vec!["AGU", "AGC"]),
//         ("tyrosine", vec!["UAU", "UAC"]),
//         ("tryptophan", vec!["UGG"]),
//         ("glutamine", vec!["CAA", "CAG"]),
//         ("asparagine", vec!["AAU", "AAC"]),
//         ("histidine", vec!["CAU", "CAC"]),
//         ("glutamic acid", vec!["GAA", "GAG"]),
//         ("aspartic acid", vec!["GAU", "GAC"]),
//         ("lysine", vec!["AAA", "AAG"]),
//         ("arginine", vec!["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"]),
//         ("stop codon", vec!["UAA", "UAG", "UGA"]),
//     ];
