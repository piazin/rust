// dentro de io temos metodos para interagir com o terminal
use std::io;

pub fn string() {
    // https://youtu.be/NKBNEJf-w70?t=1480

    // Heap
    // Heap String
    // String Dinamica
    // String 
    let mut s = String::new();
    // posso adcionar um char com o metodo push
    s.push('L');

    // posso adicionar uma string com o metodo push_str
    s.push_str("ucas");

    println!("{}", s);

    // para facilitar a criação de uma string podemos usar o metodo to_string
    let s2: String = "Souza".to_string();

    let s3 = String::from("Fernando");

    println!("{}", s3);
    println!("{}", s2);

    // podemos usar o metodo from_iter para criar uma string apartir de um array
    let nome = ['h', 'a', 'm', 'u'];
    let s4 = String::from_iter(nome);
    println!("{}", s4);

    // podemos usar o metodo into();
    // para usar este metodo é necessario que o tipo que queremos converter implemente o trait From
    let s5: String = "Milton".into();
    println!("{}", s5);

    console_input();
}

fn console_input() {
    let mut s = String::new();
    println!("Digite um texto");

    // read_line retorna um Result = Result é um enum que pode ser Ok ou Err
    // stdin() retorna um tipo io::Stdin e dentro de io::Stdin temos o metodo read_line
    // read_line() recebe com parametro uma referencia mutavel para uma string &mut s
    // expect() é um metodo que retorna o valor de Ok ou o valor de Err
    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    println!("Você digitou: {}", s);
}