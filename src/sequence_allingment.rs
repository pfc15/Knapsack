
#[derive(Clone)]
pub struct Sequence {
    pub mismatch: Vec<Vec<usize>>,
    pub matriz: Vec<Vec<usize>>,
    pub jump:usize,
    pub s1: String,
    pub s2: String,
}

impl Sequence {
    pub fn new(sequencia_1: &str, sequencia_2: &str, mismatch:Vec<Vec<usize>>) -> Self{
        let mut matriz:Vec<Vec<usize>> = Vec::new();
        for y in 0..sequencia_1.len()+1 {
            let mut linha = Vec::new();
            for x in 0..sequencia_2.len()+1 {
                if x==0 && y==0{
                    linha.push(0);
                } else if x==0 {
                    linha.push(mismatch[y-1][x]);
                } else if y==0 {
                    linha.push(mismatch[y][x-1]);
                }else {
                    linha.push(0);
                }
            }
            matriz.push(linha.clone());
        }
         Self {
            mismatch: mismatch.clone(),
            jump: 2,
            s1: String::from(sequencia_1),
            s2: String::from(sequencia_2),
            matriz: matriz.clone(),
         }
    }


    pub fn populate_matriz(&mut self, mut x:usize, mut y:usize){
        let tamanho_y = self.s1.len()+1;
        let tamanho_x = self.s2.len()+1;
        if x ==0 {
            x = x+1;
            y += 1;
        }
        if y==tamanho_y{
            return;
        }
        let cima = self.matriz[y-1][x]+self.jump;
        let esquerda = self.matriz[y][x-1]+self.jump;
        let diagonal;
        if self.s1.chars().nth(y-1).unwrap() == self.s2.chars().nth(x-1).unwrap(){
            diagonal = self.matriz[y-1][x-1];
            println!("IGUAL");
        } else{
            diagonal = self.matriz[y-1][x-1]+self.mismatch[y-1][x-1];
            println!("DIFERENTE");
        }
        
        let min = min(cima,esquerda ,diagonal );


        println!("x: {x} y: {y};  cima: {cima}; esquerda: {esquerda}; diagonal: {diagonal} min: {min}");

        self.matriz[y][x] = min;
        
        self.populate_matriz((x+1)% tamanho_x, y)

    }
}

fn min(a:usize, b:usize, c:usize) -> usize{
    if (a<b) && a<c{
        return a;
    } else if b<a && b<c {
        return b;
    }
    c
}
