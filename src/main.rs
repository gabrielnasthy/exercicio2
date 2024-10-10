// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021
use std::io;

fn ler() ->i32{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("TODO: panic message");
    input.trim().parse().unwrap()
}



fn main() {

    println!("digite um numero inteiro: ");
    let mut numero = ler();

    if numero % 3 == 0 {
        println!("esse numero é impar");
    }else if numero % 2 == 0 {
        println!("esse numero é par");
    }

    if numero < 0{
        println!("o numero é negativo");
    }else{
        println!("numero positivo")
    }

}
