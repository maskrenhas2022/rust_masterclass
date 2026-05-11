mod utils; // Certifique-se de que o arquivo utils.rs ou utils/mod.rs existe

// Você só precisa importar uma vez
use utils::terminal::{limpar_tela, exibir_menu};

fn main() {
    // 1. Limpa a tela antes de começar
    limpar_tela(); 
    
    // 2. CORREÇÃO: Removido o ":" antes do "=" e adicionado &str para os itens
    let titulo = "Menu Principal";
    let itens = ["Fundamentos", "Funções", "Tipos", "Controle"];
    let sair = false;

    // 3. Chamando a função com os argumentos corretos
    // Passamos &itens (referência) para coincidir com a assinatura &[&str]
    exibir_menu(titulo, &itens, sair);
}