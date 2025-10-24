// allowed tokens in the "gene" sequence. This list is not final; we might add specific tokens for layer types
pub const ALLOWED_TOKENS:[u32;31]=
[
	'A' as u32,// letters
	'B' as u32,
	'C' as u32,
	'D' as u32,
	'E' as u32,
	'F' as u32,
	'G' as u32,
	'H' as u32,
	'I' as u32,
	'J' as u32,
	'K' as u32,
	'L' as u32,
	'M' as u32,
	'N' as u32,
	'O' as u32,
	'P' as u32,
	'Q' as u32,
	'R' as u32,
	'S' as u32,
	'T' as u32,
	'U' as u32,
	'V' as u32,
	'W' as u32,
	'X' as u32,
	'Y' as u32,
	'Z' as u32,
	' ' as u32,	// space for separator
	';' as u32,	// semicolon for stop codon
	256,		// begin connection
	257,		// begin node
	258,		// begin layer
];
/// builds a model from the gene
pub fn build_model(gene:&[u32])->Graph<Layer<NdArray>>{
	todo!()
}

pub fn mutation_test(){
	let mut gene:Vec<u32>=vec!['H','E','L','L','O',' ','W','O','R','L','D',';'].into_iter().map(|x|x as u32).collect();
	for _ in 0..10{
		for c in gene.iter().map(|&c|char::from_u32(c).unwrap()){print!("{c}")}
		gene=mutate(gene,0.05,0.05,0.1);
		println!();
	}
}
/// at each position in the gene, possibly apply the three types of point mutations according to their respective probabilities	// TODO although this function will have a relatively low impact on performance compared to training, it could be optimized
pub fn mutate(mut gene:Vec<u32>,
              deletionchance:f32,
              insertionchance:f32,
              substitutionchance:f32
             ) ->Vec<u32>{
    let mut rng = rand::rng();
	use rand::Rng;
    use rand::seq::IndexedRandom;
    let mut y = 0;

    while y < gene.len() {
        let mut x: f32 = rng.random();
        if x < deletionchance {
            gene.remove(y);

        }

        x = rng.random();
        if x < insertionchance {
            let token = *ALLOWED_TOKENS.choose(&mut rng).unwrap();
            gene.insert(y, token);
            y = y + 1;
        }

        x = rng.random();
        if x < substitutionchance &&y<gene.len(){
            gene[y] = *ALLOWED_TOKENS.choose(&mut rng).unwrap();
        }

		y = y + 1;
    }

    gene
}
/// generates a gene that produces the model structure
pub fn transcribe_gene(model:&Graph<Layer<NdArray>>)->Vec<u32>{
	todo!()
}
/// returns true with probability chance
pub fn should_mutate(chance:f32)->bool{
	let choice:f32=rand::random();
	choice<chance
}
use block_graph::{Graph,burn::Layer};
use burn::backend::NdArray;
