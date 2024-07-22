fn dna_to_rna(dna: &str) -> String {
  
    let rna: String = dna.chars()
                      .map(|c| if c=='T'{'U'} else {c})
                      .collect();

    rna
    
}