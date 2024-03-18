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

    let by = b'A';

    // float number default f64
    // let fx = 42.1;

    // bool
    // let boolx = true;

    // https://youtu.be/IEFrj4znVIU?t=816

    println!("by {}", by);
}
