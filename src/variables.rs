pub fn variables() {
    /* ref imutáveis
    let x = 10;
    let y = &x;
    */

    let mut x = 20;
    let y = &mut x;
    *y += 5;

    println!("x {}", x);
}
