pub fn importar_funcao() {
    println!("Funcao teste");
}

pub struct Pessoa {
    pub nome: String,
    pub sobrenome: String,
}

impl Pessoa {
    pub fn mostrar_nome(&self) {
        println!("O meu nome e {}", self.nome);
    }

    pub fn nome_completo(&self) {
        println!("O meu nome completo e {} {}", self.nome, self.sobrenome);
    }
}
