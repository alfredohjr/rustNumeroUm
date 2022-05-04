// estudando o codigo RUST, um pouco por dia!
// estou utilizando o link https://doc.rust-lang.org/book/title-page.html para os estudos.

fn main() {
    //Antigo.
    println!("NumeroUm");
    println!("Ola");


    // 04/05/2022
    // sobre imutabilidade de variaveis
    let mut x = 10; // o mut indica que a variavel pode ser mutavel ao longo da execução
    println!("O valor de x é:{}", x);
    x = 12;
    println!("O valor de x é:{}", x);

    // Declarando constantes
    const Y:u32 = 10 * 40;
    println!("O valor de Y é:{}", Y);

    // usando let... dentro e fora de escopos
    let x2 = 2;
    let x2 = x2 + 1;
    {
        let x2 = x2 * 2;
        println!("o valor de x2 dentro do escopo é:{}", x2);
    }
    println!("o valor de x2 fora do escopo é:{}",x2);

    const U_POSITIVE:u8 = 1;
    const U_NEGATIVE:i8 = -1;

    // variaveis u não podem ser negativas, variaveis i sim!
    println!("o valor U_POSITIVE é:{}", U_POSITIVE);
    println!("o valor U_NEGATIVE é:{}", U_NEGATIVE);

    // calculos
    let soma = 1 + 2;
    println!("o valor de soma é:{}", soma);

    // subtração
    let subtracao_float = 30.5 - 4.3;
    println!("o valor de subtracao_float é:{}", subtracao_float);

    // multiplicacao
    let multiplicacao = 4 * 30;
    println!("o valor de multiplicacao é:{}", multiplicacao);

    // divisão
    let divisao = 40 / 3;
    println!("o valor de divisao é:{}", divisao);

    // modulo... eu acho!
    let modulo = 43 % 4;
    println!("o valor de modulo é:{}", modulo);

    // valores booleanos
    let vf = true;
    let vfe:bool = false;

    println!("{} e {}",vf,vfe);

    // tuplas e arrays
    let tup1:(i32, f64, u8) = (500, 6.4, 1);
    let (x1, y1, z1) = tup1;
    println!("a tup1 é:{},{},{}", x1, y1, z1);

    let tup2 = (500, 6.5, 2);
    let (x2, y2, z2) = tup2;
    println!("a tup2 é:{},{},{}",x2, y2, z2);

    // ou ainda
    println!("acessando valores especificos da tuple:{},{},{}", tup2.0, tup2.1, tup2.2);

    // parei aqui: https://doc.rust-lang.org/book/ch03-02-data-types.html
    // na parte de array.
}
