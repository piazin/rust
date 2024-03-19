
// Variavel estatica que pode ser acessada de qualquer lugar do projeto
// é alocada na memoria static
// o tempo de vida é o mesmo que o programa 
static _Y: u32 = 22;

pub fn memories() {

    // Variaveis que são alocadas na memoria stack
    // são alocadas em ordem de execução
    // o tempo de vida é o mesmo que a função
    // let x = 10;
    // let y = true;

    // https://youtu.be/Qezouqaoxlo?t=580

    // Variaveis que são alocadas na memoria heap
    // o tempo de vida pode ser definido manaualmente ou pelo compilador
    // let users = get_users();
}