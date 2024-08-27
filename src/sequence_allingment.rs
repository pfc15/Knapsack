
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
        for y in 0..sequencia_1.len()+2 {
            let mut linha = Vec::new();
            for x in 0..sequencia_2.len()+2 {
                if x==0 || y ==0{
                    linha.push(mismatch[y-1][x-1]);
                } else {
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
        let tamanho_y = self.s1.len()+2;
        let tamanho_x = self.s2.len()+2;
        if y==tamanho_y{
            return;
        }
        if x ==0 {
            x = x+1;
            y += 1;
        }

        self.matriz[y][x] = min(self.matriz[y-1][x]+self.jump, self.matriz[y-1][x-1]+self.mismatch[y-1][x-1], self.matriz[y][x-1]+self.jump);
        
        self.populate_matriz((x+1)% tamanho_x, y)

    }
}

fn min(a:usize, b:usize, c:usize) -> usize{
    if a>b && b<c{
        return b;
    } else if a>b && c<b {
        return c;
    }

    a
}
