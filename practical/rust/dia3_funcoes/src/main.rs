// src/main.rs

// Nossa função principal, o ponto de entrada do programa.
fn main() {
	let base1 = 'G';
	let base2 = 'C';
	let base3 = 'T';
	let base4 = 'A';

	// Chamamos nossa função para cada base e guardamos o resultado.
	let rna1 = transcrever_base(base1);
	let rna2 = transcrever_base(base2);
	let rna3 = transcrever_base(base3);
	let rna4 = transcrever_base(base4);

	println!("A transcrição de {} é {}", base1, rna1);
	println!("A transcrição de {} é {}", base2, rna2);
	println!("A transcrição de {} é {}", base3, rna3);
	println!("A transcrição de {} é {}", base4, rna4);
	
}

/// Esta função recebe uma base de DNA e retorna a base de RNA correspndente.
/// Parâmetro 'base_dna' é do tipo char.
/// O valor de retorno '->' também é do tipo char.
fn transcrever_base(base_dna:char) -> char {
	// Usamos a expressão 'match' para comparar a base de entrada.
	//  'match' é muito poderoso em Rust e o veremos em mais detalhes.
	match base_dna {
		'G' => 'C',
		'C' => 'G',
		'T' => 'A', // Em uma transcrição real, T -> A
		'A' => 'U', // A -> U é a regra de transcrição para RNA
		// O '_' é um coringa para qualquer outro caso.
		_ => 'N',
		}
		// Note que todo o bloco 'match' é uma EXPRESSÃO.
		// O valor do braço que for executado será o valor de retorno desta função.
		// por isso, não há ponto e vírgula aqui.
}		
