use rpassword::prompt_password;


pub  fn limpar_tela(){

   print!("{esc}c", esc = 27 as char);

}