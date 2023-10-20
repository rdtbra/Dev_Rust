fn loop_infinito() {
   let mut a = 0;
   loop {
      if a == 5 {
        break;
      }
      println!("{:?}", a);
      a = a + 1;
   }
}

fn loop_while() {
    let mut a = 0;
    while a != 5 {
        println!("{:?}", a);
        a = a + 1;
    }
}

fn soma(a: i32, b:i32) -> i32 {
    a + b
}

fn uso_match(teste: &str) {
    match teste {
        "Reinaldo" => {
            println!("Boa Noite!");
            println!("Reinaldo");
        },
        "Rinaldo" => println!("Rinaldão!"),
        _ => println!("Será que deu certo!"),
    }
}

fn main() {

    println!("Hello, world!");
    loop_infinito();
    loop_while();
    let soma = soma(10,35);
    println!("Soma {:?}", soma);
    let nome = "Rinaldo";
    uso_match(nome);
    
}
