fn main() {
    // --- Exemplo com o trait 'Copy' (stack) ---
    println!("--- Testando o Comportamento de Cópia (Copy) ---");
    let x = 100; // x é do tipo i32, que implementa 'Copy'
    let y = x; // O valor de x é copiado para y.

    // Ambas as variáveis continuam válidas e podem ser usadas.
    println!("O valor de x é: {}", x);
    println!("O valor de y é: {}", y);
    println!("O valor de x: {} e y: {}", x, y);
    println!("\n---------------------------------------------\n");

    // --- Exemplo com a semântica de 'Move' (tipo do heap) ---
    println!("--- Testando o comportamento de Mover (Move) ---");
    let s1 = String::from("GATTACA"); // s1 é dona da String "GATTACA"
    let s2 = s1;                      // A posso da String é MOVIDA de s1 para s2.

    // s1 não é mais a dona e foi invalidada para garantir a segurança da memória.

    // A linha abaixo, se descomentanda, causará um erro de compilação!
    // TENTE DESCOMENTAR PARA VER A MENSAGEM DO COMPILADOR!
    // println!("O valor de s1 é: {}", s1);

    // Apenas s2, a nova dona, pode ser usada.
    println!("O valor de s2 é: {}", s2);
    println!("Apenas a nova dona (s2) é válida após a atribuição.");
}
