use rpassword::prompt_password;

pub fn exibir_menu(titulo: &str, itens: &[&str], _sair: bool) -> u32 {
    limpar_tela();
    
    let completo = format!("Masterclass Rust::{}", titulo);
    println!("{}", completo);
    println!("{}", "=".repeat(completo.len()));

    // O loop abaixo já exibe os itens. 
    // NÃO precisamos de uma função extra 'exibir_itens' aqui.
    for (i, item) in itens.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }

    0 // Retorna 0 apenas como um placeholder
}

pub fn esperar_enter() {
    let _ = prompt_password("Pressione ENTER para continuar...").unwrap();
}

pub fn limpar_tela() {
    print!("{esc}c", esc = 27 as char);
}