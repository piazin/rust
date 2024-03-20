use std::io;

pub fn calculations() {
    // {:-^40}
    // o :  indicar que a formatação está começando 
    // o "-" indica o caracter que vai ser repetido
    // o ^ indica que o caracter vai ser repetido no centro
    // o 40 indica a quantidade de vezes que o caracter vai ser repetido
    println!("{:-^40}", "Calculator");

    let banner = 
    "Digite numeros \n\
     separados por virgula para somar";
    println!("{banner}");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Erro reading line");

    /* 
    no lugar de criar uma função, você pode usar uma função anonima 
    em js (x) => x.trim() em rust |x| x.trim()
    .map(clean)
    fn clean(c: &str) -> &str {
        return c.trim();
    }
    */    

    let nums: Vec<i32> = user_input
        .split(",")
        .map(|x| x.trim().parse().expect("Error"))
        .collect();

    let result: i32 = nums.iter().sum();


    println!("o total é {}", result);
    // println!("nums {:?}", nums);

    println!("{}", "-".repeat(40))
}