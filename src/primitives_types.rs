pub fn primitives_types() {
    // int default i32 range (-2147483648 até 2147483647)
    // let xi = 30;

    // let xu: u8 = 30;

    // podemos também inferir o tipo dessa maneira com o valor_tipo
    // let y = 127_u8;

    //permite adcionar _ em numeros muitos grandes para facilitar a leitura
    // let yunderline = 199_456_898_9;

    // let h = 0xff; // hexadecimal

    /* (1 * 128) + (1 * 64) + (1 * 32) + (1 * 16) + (0 * 8) + (0 * 4) + (0 * 2) + (0 * 1)
        = 128 + 64 + 32 + 16
        = 240
    */
    // let b = 0b1111_0000;

    // byte
    // let by = b'A';

    // float number default f64
    // let fx = 42.1;

    // bool
    // let boolx = true;

    // https://youtu.be/IEFrj4znVIU?t=816

    // println!("by {}", by);

    letra();
    tuplas();
    arrays();

}

fn letra() {
    // aceita unicode até 4 bytes
    let letra = 'a';
    let emoji = '😻';

    println!("letra {}", letra);
    println!("emoji {}", emoji);
}

fn tuplas() {
    // tuplas
    // uma tupla pode ter varios tipos de dados
    // uma vez definida a tupla não pode ser alterada
    let mut numbers = (1, 2, 3.5);
    println!("number {:?}", numbers);
    println!("number 0 {}", numbers.0);

    // alterando o valor da tupla
    numbers.0 = 10;

    // desestruturação
    let (a, b, c) = numbers;

    // tupla mutavel
    numbers = (4, 4, 4.5);

    println!("a {} b {} c {}", a, b, c);
    println!("replace numbers {:?}", numbers);
}

fn arrays() {
    // rust não permite arrays de tipos diferentes
    let numbers = [1, 2, 3, 4, 5];

    println!("numbers {:?}", numbers[4]);
    
    // index out of bounds error
    // println!("numbers {:?}", numbers[5]);

    // tamanho do array
    println!("numbers length {}", numbers.len());

    // slice
    let slice = &numbers[0..3];
    println!("slice {:?}", slice);
}