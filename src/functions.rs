#[allow(unused, dead_code)]
pub fn functions() {
    // say_hello("Lucas");

    // Block Expressions
    // Unit Type
    let y: () = {
        say_hello("Hamu");
        let x: i32 = 5;

        // para retornar 99 não ultilizamos o ;
        // 99
    };
    println!("{:?}", y);

    let result = add_numbers(8, 9);
    println!("{}", result);

    closures();
}

fn say_hello(name: &str) {
    println!("Hello {name}");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    if x == 0 {
        // return é mais ultilizado em clausulas guardas
        return  y;
    }

    // expresion é tudo que crie um valor de retorno igual ao exemplo:
    // return  x + y;
    // a ultima expressão sem ; antes de terminar a função vai ser o retorno
    // não é necessario usar return em rust
    x + y 
}

fn convert_to_number(item: &str) -> i32 {
    let number: i32 = item.parse().unwrap();
    number
}

// fn double_number(n: i32) -> i32 {
//     n * 2
// }

fn closures() {
    let input: &str = "56 65 58 48 59 56 87 23";

    let numbers: Vec<i32> = input
                                .split(" ")
                                .map(|i| convert_to_number(i))
                                //.map(double_number)
                                // use closures
                                .map(|n| n * 2)
                                .collect();

    println!("{:?}", numbers);
}
// https://youtu.be/jNaVT19Pwd0?list=LL&t=875