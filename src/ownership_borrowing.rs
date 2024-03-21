/*
    # O que o Borrow Checker Resolve?

    - Null Pointer Exception - é quando você tenta acessar um ponteiro nulo.
    - Segmentation Fault - é quando você tenta acessar uma área de memória que não foi alocada para o seu programa.
    - Memory Leak - é quando você aloca memória e não libera, ou seja, você perde a referência para a memória alocada.
    - Dangling Pointer - é quando você libera a memória e continua com um ponteiro que aponta para a memória liber
    - Double free - é quando você libera a memória duas vezes.
    - Use after free - é quando você tenta acessar memória que já foi liberada.
    - Data Races - é quando duas threads tentam acessar a mesma memória ao mesmo tempo.
*/

#![allow(unused, dead_code)] // é uma diretiva de atributo essa linha específica é usada para suprimir avisos do compilador sobre código não utilizado e código morto (dead code).
pub fn ownership() {
    let a: i32 = 10; // Copy values (i32, f64, bool, char, int) // Stack memory
    let b = a; // O rust vai relizar uma copia e não um ref
    let c: &i32 = &a; // c é uma refencia a let a usando &a

    println!("O valor de a é {a}");
    println!("O valor de b é {b}");
    // print pointer
    println!("O valor de c é {:p}", c); //https://stackoverflow.com/a/27852760/19552432

    // https://chat.openai.com/share/ad5df033-0dbc-4096-a7ce-74153494f82b
    let s: String = String::from("Hamu"); // No Copy
    let s1: String = s.clone(); // clone value
                                // let s2: String = s; // move value to b transferencia de dono de s para s2
    let s3: &String = &s; // referencia ao valor de s um emprestimo

    println!("O valor de s: {}", s);
    println!("O valor de S1: {}", s1);
    // println!("O valor de S2: {}", s2);

    let mut name = "Milton".to_string();

    //say_hello(&name); // Borrow - Ref
    //say_goodbye(&name);

    add_prefix(&mut name);

    // Podemos ter uma unica referencia caso a a variavel seja mutavel
    // E varias caso seja imutavel
    to_upper_case(&mut name);

    // em vez de passar uma ref podemos usar a treating clone() que vai realizar a copia do valor
    say_hello_clone(name.clone());
}

fn to_upper_case(text: &mut String) {
    // usamos o * para dereferenciar a referencia text
    *text = text.to_uppercase();
}

fn add_prefix(text: &mut String) {
    *text = format!("FOO_{text}");
    // text.push_str("_FOO"); // em alguns casos o rust já realiza a dereferencia
}

fn say_hello_clone(text: String) {
    println!("Clone Hello {text}");
}

fn say_hello(text: &String) {
    println!("Hello {text}");
}

fn say_goodbye(text: &String) {
    println!("Goodbye {text}");
}

/*
// text recebe o valor de name agora ele é dono
// sendo assim não é possivel usar em say_goodbye(name)
fn say_hello(text: String) {
    println!("Hello {text}");
}

fn say_goodbye(text: String) {
    println!("Goodbye {text}");
}
*/

