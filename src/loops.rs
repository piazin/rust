pub fn loops() {
    // While
    let mut counter = 0;

    while counter < 10 {
        println!("{counter}");
        counter += 1;
    }

    stop_loops();
}

fn stop_loops() {
    'outer: for i in 1..=3 {
        println!("Começando iteração externa {}", i);

        'inner: for j in 1..=3 {
            println!("Começando iteração interna {}", j);

            if i == 1 && j == 1 {
                println!("Continuando para a próxima iteração externa");
                continue 'outer; // Pula para a próxima iteração do loop externo
            }

            if i == 2 && j == 2 {
                println!("Parando loop externo!");
                break 'outer; // Para o loop externo 'outer
            }
        }
    }
}
