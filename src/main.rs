// use std::io; // BIBLIOTECA QUE FAZ LEITURA DE INPTUS DO USUARIO

// fn convert_string_to_int(string: &String) -> i32 {
//     let string_converted_to_int = string.trim().parse::<i32>().unwrap();
//     string_converted_to_int
// }

// ---------------- RECEBE DOIS INPUTS DO USUARIO E VERIFICA QUAL E MAIOR ----------------
// fn main() {
//     let mut number1 = String::new();
//     io::stdin()
//         .read_line(&mut number1)
//         .expect("Erro ao ler number1");

//     let mut number2 = String::new();
//     io::stdin()
//         .read_line(&mut number2)
//         .expect("Erro ao ler number2");

//     if convert_string_to_int(&number1) > convert_string_to_int(&number2) {
//         println!("O numero {} e maior que {}", number1, number2);
//     } else {
//         println!("O mumero {} e menor ou igual que {}", number1, number2);
//     }
// }

// ---------------- CALCULAR A SOMA DE DIGITOS ----------------
// fn main() {
//     let mut soma_final = 0;
//     let mut valor_entrada = String::new();
//     io::stdin()
//         .read_line(&mut valor_entrada)
//         .expect("Erro ao ler input");

//     let mut valor_inteiro = convert_string_to_int(&valor_entrada);

//     while valor_inteiro != 0 {
//         let mut resto = valor_inteiro % 10;
//         soma_final = soma_final + resto;
//         valor_inteiro = valor_inteiro / 10;
//     }

//     println!("A soma dos numeros digitados e {}", soma_final)
// }

// ---------------- CALCULAR O FATORIAL DE UM NUMERO ----------------
// fn main() {
//     let mut valor_input = String::new();
//     io::stdin()
//         .read_line(&mut valor_input)
//         .expect("Erro ao ler input");
//     let mut valor_input_inteiro = convert_string_to_int(&mut valor_input);
//     let mut fatorial = 1;

//     while valor_input_inteiro > 1 {
//         fatorial = fatorial * valor_input_inteiro;
//         valor_input_inteiro = valor_input_inteiro - 1;
//     }

//     println!("O fatorial de {} e {}", valor_input, fatorial);
// }

// ---------------- CALCULAR O NUMERO DE ALUNOS QUE FICARAM EM RECUPERACAO ----------------
// fn main() {
//     let mut numero_alunos = String::new();
//     io::stdin()
//         .read_line(&mut numero_alunos)
//         .expect("Nao foi possivel ler a quantidade de alunos");

//     let mut numero_alunos_int = convert_string_to_int(&numero_alunos);
//     let mut alunos_em_recuperacao = 0;

//     while numero_alunos_int != 0 {
//         let mut media_aluno = String::new();
//         io::stdin()
//             .read_line(&mut media_aluno)
//             .expect("Nao foi possivel ler a media do aluno");

//         if convert_string_to_int(&mut media_aluno) >= 3
//             && convert_string_to_int(&mut media_aluno) < 6
//         {
//             alunos_em_recuperacao += 1;
//         }

//         numero_alunos_int -= 1;
//     }

//     println!(
//         "O total de alunos em recuperacao e de {}",
//         alunos_em_recuperacao
//     );
// }

// ---------------- CALCULAR O MAIOR DIVISOR COMUM ENTRE 15 E 40 ----------------
// fn main() {
//     let mut a = 15;
//     let mut b = 40;

//     while b != 0 {
//         let temp = b;
//         b = a % b;
//         a = temp;
//     }

//     println!("O maior divisor comum entre 15 e 40 e {}", a);
// }

// fn dobro(num: i32) -> i32 {
//     return 2 * num;
// }

// fn maior_entre_dois_numeros(num1: i32, num2: i32) -> i32 {
//     if num1 > num2 {
//         return num1;
//     } else {
//         return num2;
//     }
// }

// fn main() {
//     println!("O dobro de 5 e {}", dobro(5));

//     print!(
//         "O maior numero entre 5 e 10 e {}",
//         maior_entre_dois_numeros(5, 10)
//     );
// }

// ---------------- TUPLAS ----------------
// fn main() {
//     let tuplas = (1, "valores", 3.14, (1, 2, 3));

//     let (a, b, c, d) = tuplas;

//     println!("O valor de a e {}", a);
//     println!("O valor de b e {}", b);
//     println!("O valor de c e {}", c);
//     println!("O valor de d e {:?}", d);
// }

// ---------------- ENUMS ----------------
// enum Direction {
//     Up,
//     Right,
//     Bottom,
//     Left,
// }

// fn main() {
//     let player: Direction = Direction::Right;

//     match player {
//         Direction::Up => println!("O jogador foi para a cima"),
//         Direction::Right => println!("O jogador foi para a direita"),
//         Direction::Bottom => println!("O jogador foi para a baixo"),
//         Direction::Left => println!("O jogador foi para a esquerda"),
//     }
// }

// #[derive(Debug)]
// enum Gender {
//     Male,
//     Female,
// }

// fn main() {
//     let player_male: Gender = Gender::Male;
//     let player_female: Gender = Gender::Female;

//     println!("{:?}", player_male);
//     println!("{:?}", player_female);
// }

// enum CarType {
//     Fiat,
//     Ford,
//     Renault,
// }

// fn nacionalidade_carro(car: CarType) {
//     match car {
//         CarType::Fiat => println!("O carro e italiano"),
//         CarType::Ford => println!("O carro e americano"),
//         CarType::Renault => println!("O carro e frances"),
//     }
// }

// fn main() {
//     nacionalidade_carro(CarType::Fiat);
//     nacionalidade_carro(CarType::Ford);
//     nacionalidade_carro(CarType::Renault);
// }

// enum Pagamento {
//     Dinheiro(f32),
//     Debito(bool, f32),
// }

// fn main() {
//     let tipo_pagamento: Pagamento = Pagamento::Debito(true, 99.90);

//     match tipo_pagamento {
//         Pagamento::Dinheiro(qty) => println!("O valor pago foi de R${}", qty),
//         Pagamento::Debito(true, qty) => println!("O valor pago foi de R${}", qty),
//         Pagamento::Debito(false, _qty) => println!("Erro ao realizar pagamento",),
//     }
// }

// ---------------- CONSTANTES ----------------
// const PI: f32 = 3.14159;

// fn comprimento_circunferencia(raio: f32) -> f32 {
//     let comprimento = 2f32 * raio * PI;
//     return comprimento;
// }

// fn main() {
//     println!(
//         "A circunferencia de raio {} possui comprimento de {}",
//         2.32,
//         comprimento_circunferencia(2.32)
//     );
// }

// ---------------- METODOS DE STRINGS ----------------
// fn main() {
//     let mut frase: String = String::from("Oi, meu nome e Jose.");

//     println!("O tamanho da frase e {}", frase.len());

//     println!("A frase esta vazia? {}", frase.is_empty());

//     println!("A frase contem a palavra Jose? {}", frase.contains("Jose"));

//     for espaco in frase.split_whitespace() {
//         println!("{}", espaco);
//     }

//     frase.push_str("Eu tenho 18 anos");
//     println!("{}", frase);
// }

// ---------------- STRUCTS ----------------
// struct User {
//     username: String,
//     email: String,
//     ativo: bool,
//     genero: String,
// }

// fn main() {
//     let mut usuario: User = User {
//         username: String::from("JoaoPessoa"),
//         email: String::from("joaopessoa@gmail.com"),
//         ativo: true,
//         genero: String::from("Masculino"),
//     };
//     usuario.ativo = false;

//     println!("O nome do usuario e {}", usuario.username);
// }

// ---------------- STRUCTS E TUPLAS ----------------
// struct User(String, String, bool, String);

// fn main() {
//     let mut usuario = User(
//         String::from("JoaoPessoa"),
//         String::from("joaopessoa@gmail.com"),
//         true,
//         String::from("Masculino"),
//     );
//     usuario.2 = false;

//     println!("O nome do usuario e {}", usuario.0);
// }

// ---------------- ARRAY ----------------
// fn main() {
//     let numeros_inteiros = [1, 2, 3, 4, 5];

//     let numeros_inteiros: [i32, 5] = [1, 2, 3, 4, 5]; // FALA QUE O ARRAY E DE INT E TEM 5 ELEMENTOS
//     let numeros_inteiros = [2; 1000]; // VAI PRINTAR 1000 VEZES O NUMERO 2

//     for numero in 0..numeros_inteiros.len() {
//         println!("{}", numeros_inteiros[numero]);
//     }

//     // VARIACAO PRA PERCORRER O ARRAY
//     for num in numeros_inteiros.iter() {
//         println!("{}", num);
//     }
// }

// ---------------- IMPLEMENTATION ----------------
// struct User {
//     username: String,
//     email: String,
//     ativo: bool,
//     genero: String,
// }

// impl User {
//     fn nome_do_usuario(&self) {
//         // O SELF VAI ACESSAR AS PROPRIEDADES DO STRUCT
//         println!("O nome do usuario e {}", self.username);
//     }

//     fn usuario_esta_ativo(&self) {
//         println!("O usuario esta ativo? {}", self.ativo);
//     }
// }

// fn main() {
//     let usuario: User = User {
//         username: String::from("JoaoPessoa"),
//         email: String::from("joaopessoa@gmail.com"),
//         ativo: true,
//         genero: String::from("Masculino"),
//     };

//     usuario.nome_do_usuario();
//     usuario.usuario_esta_ativo();
// }

// ---------------- VETORES ----------------
// fn main() {
//     let mut vetores = vec![1, 2, 3, 4];
//     // let mut vetores: Vec<i32> = Vec::new(); // OUTRA FORMA DE DECLARAR VETOR

//     vetores.push(5);
//     println!("{:?}", vetores);

//     vetores.remove(1);
//     println!("{:?}", vetores);

//     for index in vetores.iter() {
//         println!("{}", index);
//     }

//     for index in 0..vetores.len() {
//         println!("{}", vetores[index]);
//     }
// }

// ---------------- HASH MAP ----------------
// use std::collections::HashMap;

// fn main() {
//     let mut hash_map = HashMap::new();

//     hash_map.insert("Historia", 96); // INSERE VALORES NO HASH_MAP
//     hash_map.insert("Portugues", 89);
//     hash_map.insert("Geografia", 75);

//     println!("O aluno estudou quantas materias? {}", hash_map.len());

//     match hash_map.get("Portugues") {
//         Some(key) => println!("O aluno tirou nota {} em Portugues", key),
//         None => println!("O aluno nao estudou Portugues"),
//     }

//     hash_map.remove("Geografia");
//     println!(
//         "O aluno estudou Geografia? {}",
//         hash_map.contains_key("Geografia")
//     );

//     println!(
//         "O aluno estudou Historia? {}",
//         hash_map.contains_key("Historia")
//     );
// }

// ---------------- COMO LER ARQUIVOS ----------------
// use std::fs::File; // PERMITE IMPORTAR O ARQUIVO .TXT
// use std::io::prelude::*; // PERMITE LER O ARQUIVO .TXT

// fn main() {
//     let mut arquivo = File::open("./src/rust_wiki.txt").expect("Nao foi possivel ler o arquivo");
//     let mut conteudo = String::new();

//     arquivo
//         .read_to_string(&mut conteudo)
//         .expect("Nao foi possivel ler o conteudo do arquivo");

//     println!("O conteudo do arquivo e: \n\n {}", conteudo);
// }

// ---------------- COMO CRIAR ARQUIVOS ----------------
// use std::fs::File;
// use std::io::prelude::*;

// fn main() {
//     let mut arquivo =
//         File::create("./src/criar_teste.txt").expect("Nao foi possivel criar o arquivo");

//     arquivo
//         .write_all(b"Criando um txt teste") // O b SIGNIFICA BITE SLICE, EH REFERENTE A STRING
//         .expect("Nao foi possivel adicionar conteudo ao txt");
// }

// ---------------- DEFININDO TRAITS DE UM STRUCT ----------------
// TRAITS SAO METODOS QUE SERAO APLICADOS NO STRUCT
// struct Pessoa {
//     nome: String,
//     idade: i32,
// }

// trait Acao {
//     fn falar(&self);

//     fn pode_votar(&self) -> bool;
// }

// // NO IMPLEMENTS EU COLOCO O CORPO DOS METODOS INFORMADOS NO TRAIT
// impl Acao for Pessoa {
//     fn falar(&self) {
//         println!("Ola, meu nome e {}", self.nome);
//     }

//     fn pode_votar(&self) -> bool {
//         if self.idade >= 18 {
//             return true;
//         }

//         return false;
//     }
// }

// fn main() {
//     let pessoa = Pessoa {
//         nome: String::from("Joao"),
//         idade: 22,
//     };

//     pessoa.falar();
//     println!("{} pode votar? {}", pessoa.nome, pessoa.pode_votar());
// }

// ---------------- MATCH ----------------
// fn main() {
//     let numero = 5;

//     match numero {
//         1 => println!("O numero e 1"),
//         2 | 3 => println!("O mumero e 2 ou 3"),
//         4..=10 => println!("O numero esta entre 4 e 10"),
//         _ => println!("O numero e 11 ou maior"),
//     }

//     let nome = "Joao";

//     match nome {
//         "Joao" => println!("O nome e Joao"),
//         "Marcos" => println!("O nome e Marcos"),
//         _ => println!("Nao consigo saber seu nome"),
//     }
// }

// ---------------- INPUT DE DADOS COM O MATCH ----------------
// use std::io;

// fn main() {
//     let mut mensagem_usuario = String::new();

//     println!("Digite sua mensagem:");
//     match io::stdin().read_line(&mut mensagem_usuario) {
//         Ok(_) => println!("A mensagem digitada foi: {}", mensagem_usuario),
//         Err(e) => println!("Encontramos um erro ao ler sua mensagem. {}", e),
//     }
// }

// ---------------- SLICE STRING ----------------
// fn main() {
//     let hello_world = String::from("Hello World");
//     let hello = &hello_world[0..5];

//     println!("{}", hello_world);
//     println!("{}", hello);
// }

// ---------------- METODOS STRING ----------------
// fn main() {
//     {
//         let nome = String::from("Oi, meu nome e Joao");
//         println!("{}", nome);
//         println!("{}", nome.replace("Joao", "Marcos"));
//     }

//     // METODO LINES FUNCIONA EM STRINGS COM MAIS DE UMA LINHA, E PERMITE MUDAR TODAS AS LINHAS DE UMA VEZ SO
//     {
//         let nome = String::from("Oi\nmeu nome e\nJoao");

//         for line in nome.lines() {
//             println!("( {} )", line);
//         }
//     }

//     {
//         let nome = String::from("Oi, meu nome e Joao");
//         let vetor: Vec<&str> = nome.split(" ").collect(); // APLICA O SPLIT PRA CORTAR A FRASE E DEPOIS O COLLECT PRA TRANSFORMAR EM VETOR

//         println!("{:?}", vetor);
//         println!("{}", vetor[1]);
//     }

//     {
//         let nome = String::from("   Oi, meu nome e Joao   ");
//         println!("{}", nome);
//         println!("{}", nome.trim());

//         let nome2 = String::from("Oi,    meu nome e   Joao");
//         println!("{}", nome2);
//         println!("{}", nome2.trim());
//     }

//     {
//         // BUSCAR UM CARACTER DENTRO DE UMA STRING
//         let nome = String::from("Oi, meu nome e Joao");

//         match nome.chars().nth(4) {
//             Some(char) => println!("O caracter da 4 posicao e: {}", char),
//             None => println!("Erro ao buscar caracter"),
//         }
//     }
// }

// ---------------- COMO CRIAR NUMERO RANDOMICOS ----------------
// IR NO CARGO.TOML E ADICIONAR UMA LINHA NO FINAL: rand = "0.3"
// extern crate rand;
// use rand::Rng;

// fn main() {
//     // O thread_rng VAI GERAR OS NUMEROS E O gen_range VAI FALAR QUAL O RANGE DOS NUMEROS GERADOS
//     let numeros_randomicos = rand::thread_rng().gen_range(1, 10);
//     println!("{}", numeros_randomicos);

//     // FLOAT NUMBERS
//     let numeros_randomicos2 = rand::thread_rng().gen_range(1., 10.);
//     println!("{}", numeros_randomicos2);

//     // BOOLEAN
//     // O NUMERO EH A PORCENTAGEM DE CHANCE DE DAR TRUE. 2 SIGNIFICA 50%
//     let true_false = rand::thread_rng().gen_weighted_bool(2);
//     println!("{}", true_false);
// }

// ---------------- IMPORTAR FUNCOES DE OUTROS ARQUIVOS ----------------
// mod importar;

// fn main() {
//     importar::importar_funcao();
// }

// ---------------- IMPORTAR STRUCTS E IMPL DE OUTROS ARQUIVOS ----------------
// mod importar;

// fn main() {
//     let usuario = importar::Pessoa {
//         nome: String::from("Joao"),
//         sobrenome: String::from("Pessoa"),
//     };

//     usuario.mostrar_nome();
//     usuario.nome_completo();
// }

// ------------------------------------------------
// ---------------- EXERCICIOS ----------------

// ---------------- 1. CALCULAR A MEDIA, MEDIANA E MODA DE UM VETOR ----------------
// use std::collections::HashMap;

// fn media(numeros: &Vec<i32>) -> f64 {
//     let mut soma_numeros_vetor = 0;

//     for numero in numeros {
//         soma_numeros_vetor += numero;
//     }

//     return soma_numeros_vetor as f64 / numeros.len() as f64;
// }

// fn mediana(numeros: &Vec<i32>) -> f64 {
//     let mut numeros_em_ordem_crescente = numeros.clone(); // CRIA UM CLONE DO VETOR PRA TRABALHAR COM ELE
//     numeros_em_ordem_crescente.sort(); // DEIXA O VETOR EM ORDEM CRESCENTE

//     let numero_centro = numeros.len() / 2; // DIVIDE O VETOR EM 2 E PEGA O NUMERO DO MEIO. SE O LEN FOR IMPAR ESSA EH A MEDIANA

//     // SE FOR PAR, ELE VAI PEGAR O NUMERO DO CENTRO, E UM NUMERO ANTES DO CENTRO PRA PODER FAZER A MEDIA DOS DOIS E ACHAR A MEDIANA
//     if numeros_em_ordem_crescente.len() % 2 == 0 {
//         return media(&vec![
//             numeros_em_ordem_crescente[numero_centro],
//             numeros_em_ordem_crescente[numero_centro - 1],
//         ]);
//     }

//     return numeros_em_ordem_crescente[numero_centro] as f64;
// }

// fn moda(numeros: &Vec<i32>) -> i32 {
//     let mut map_numeros = HashMap::new(); // CRIA UM ESPACO NA MEMORIA PRA ESSA VARIAVEL

//     // VAI CONTAR QUANTAS VEZES CADA NUMERO SE REPETE DENTRO DO VETOR
//     // SE EH A PRIMEIRA VEZ QUE O NUMERO APARECER, ELES VAI PRO or_insert E COMECA COM 0
//     for num in numeros {
//         let cont = map_numeros.entry(num).or_insert(0);
//         *cont += 1;
//     }

//     let mut numero_que_mais_repete = 0;
//     let mut key_que_mais_repete = 0;

//     // FAZ O FOR PRA SABER QUAL E O NUMERO QUE MAIS REPETE NO VETOR
//     for (key, value) in map_numeros {
//         if value > numero_que_mais_repete {
//             numero_que_mais_repete = value;
//             key_que_mais_repete = *key;
//         }
//     }

//     return key_que_mais_repete;
// }

// fn main() {
//     let vetor = vec![1, 5, 3, 3, 5, 2, 4];

//     println!("A media e: {}", media(&vetor));

//     println!("A mediana e: {}", mediana(&vetor));

//     println!("A moda e: {}", moda(&vetor));
// }

// ---------------- 2. PIG LATIN ----------------
// fn converte_pra_pig_latin(palavra: &String) -> String {
//     let vogais = ["a", "e", "i", "o", "u"];
//     let (primeira_letra, resto_palavra) = palavra.split_at(1); // VAI CORTAR A PRIMEIRA LETRA DA PALAVRA
//     let primeira_letra_vogal = vogais.contains(&primeira_letra); // VERIFICA SE vogais TEM A primeira_letra

//     if primeira_letra_vogal {
//         return format!("{}-hay", resto_palavra);
//     }

//     return format!("{}-{}ay", resto_palavra, primeira_letra);
// }

// fn main() {
//     let palavra = String::from("time");
//     let palavra2 = String::from("amor");

//     println!(
//         "A palavra em pig-latin e: {}",
//         converte_pra_pig_latin(&palavra)
//     );

//     println!(
//         "A palavra em pig-latin e: {}",
//         converte_pra_pig_latin(&palavra2)
//     );
// }

// ---------------- 3. ADICIONAR PESSOA AO DEPARTAMENTO ----------------
// use std::{collections::HashMap, io};

// fn departamento() {
//     let mut departamento_pessoas = HashMap::new();

//     loop {
//         println!("Digite: Adicione <Pessoa> para <Departamento>");
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("Erro na leitura");

//         let input = input.trim(); // REMOVE OS ESPACOS ANTES E DEPOIS DO COMANDO
//         let mut input_vetor = input.split_whitespace(); // SEPARA O COMANDO POR ESPACOS

//         // VERIFICA SE FOI ACHADO O NOME DA PESSOA NA POSICAO 1 DO VETOR
//         let nome_pessoa = match input_vetor.nth(1) {
//             Some(pessoa) => pessoa,
//             None => {
//                 println!("Nao foi possivel atribuir <Pessoa>");
//                 continue;
//             }
//         };

//         // QUANDO SE USA O nth, ELE "CONSOME" A STRING, ENTAO COMO FOI USADO ANTES, AS 2 PRIMEIRAS POSICOES FORAM CONSUMIDAS
//         // VERIFICA SE FOI ACHADO O NOME DA PESSOA NA POSICAO 1 DO VETOR
//         let nome_departamento = match input_vetor.nth(1) {
//             Some(departamento) => departamento,
//             None => {
//                 print!("Nao foi possivel atribuir <Departamento>");
//                 continue;
//             }
//         };

//         // ADICIONA O NOME DO DEPARTAMENTO COMO KEY NO HASH MAP, SE FOR A PRIMEIRA VEZ ELE VAI CRIAR UM VETOR
//         let empregado = departamento_pessoas
//             .entry(String::from(nome_departamento))
//             .or_insert(vec![]);
//         empregado.push(String::from(nome_pessoa));

//         println!("{:?}", departamento_pessoas);
//     }
// }

// fn main() {
//     departamento();
// }
