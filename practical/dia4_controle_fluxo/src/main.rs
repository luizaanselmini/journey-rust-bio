// src/main.rs

fn main() {
	let conteudo_gc1 = 0.35; // Conteúdo GC baixo
	let conteudo_gc2 = 0.55; // Conteúdo GC médio
	let conteudo_gc3 = 0.71; // Conteúdo GC alto

	println!("Analisando conteúdo GC:");
	analisar_conteudo_gc(conteudo_gc1);
	analisar_conteudo_gc(conteudo_gc2);
	analisar_conteudo_gc(conteudo_gc3);

	println!("\nAnalisando tamanho de genes:");
	let tamanho_gene1 = 150;    // Pequeno
	let tamanho_gene2 = 2500;   // Médio
	let tamanho_gene3 = 15000;	// Grande

	//Chamamos a função e guardamos a classificação retornada.
	let classificao1 = classificar_tamanho_gene(tamanho_gene1);
	let classificao2 = classificar_tamanho_gene(tamanho_gene2);
	let classificao3 = classificar_tamanho_gene(tamanho_gene3);

	println!("O gene de {} pares de base é considerado '{}'", tamanho_gene1, classificao1);
	println!("O gene de {} pares de base é considerado '{}'", tamanho_gene2, classificao2);
	println!("O gene de {} pares de base é considerado '{}'", tamanho_gene3, classificao3);

}	
	/// Usa  'if/else' como declaração para imprimir uma análise.
	/// Esta função retorna um valor.
	fn analisar_conteudo_gc(gc: f64) {
		if gc > 0.65 {
			println!(" - Conteúdo GC de {} é considerado ALTO.", gc);
		} else if gc > 0.45 {
			println!(" - Conteúdo GC de {} é considerado MÉDIO", gc);
		} else {
			println!(" - Conteúdo GC de {} é considerado BAIXO", gc);
		}

	}

/// Usa 'if/else' como uma expressão para retornar um valor.
/// O valor de retorno é um '&'static str', uma referência a uma string que vive por toda a duração de um programa.
fn classificar_tamanho_gene(comprimento: u32) -> &'static str {
	// A variável  'classificação' recebe o valor da expressão 'if'.
	let classificacao = if comprimento > 10000 {
		"Grande"
	} else if comprimento > 2000 {
		"Médio"
	} else {
		"Pequeno"
	};
	// Retorna o valor que foi atribuído.
	classificacao
}