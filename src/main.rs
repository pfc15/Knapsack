use std::time::Instant;

fn main() {
    let lista = [Item{
        value: 1,
        weight:1,
    }, Item{
        value:6,
        weight:2,
    }, Item {
        value:18,
        weight:5
    }, Item {
        value: 22,
        weight: 6
    }, Item {
        value:28,
        weight: 7
    }].to_vec();
    
    let now = Instant::now();
    let mut classe = Knapsack::new( lista);
    let y = classe.items.len();
    println!("resposta: {}", classe.solve(11, y));
    let elapsed = now.elapsed();
    for y in 0..classe.items.len()+1{
        for x in 0..12{
            print!(" {} |", classe.matriz[y][x].valor);
        }
        println!("");
    }

    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    let caminho = classe.encontrar_caminho(11, y, Vec::new() as Vec<Item>);
    let elapsed = now.elapsed();
    for i in caminho.into_iter().rev() {
        println!("valor: {}, peso: {}", i.value, i.weight);
    }
    println!("Elapsed: {:.2?}", elapsed);
}

#[derive(Clone)]
struct Item {
    value: usize,
    weight: usize
}

#[derive(Clone)]
struct No {
    valor: usize,
    visitado: bool
}

#[derive(Clone)]
struct Knapsack {
    items: Vec<Item>,
    matriz: Vec<Vec<No>>,
}

impl Knapsack {
    fn new(items:Vec<Item>) -> Self{
        let mut matriz:Vec<Vec<No>> = Vec::new();
        for y in 0..items.len()+1{
            let mut linha:Vec<No> = Vec::new();
            for x in 0..11+1{
                if x==0 || y==0{
                    let no_add = No {
                        valor:0,
                        visitado: true,
                    };

                    linha.push(no_add);
                }else {
                    let no_add = No {
                        valor:0,
                        visitado: false,
                    };

                    linha.push(no_add);
                }
            }
            matriz.push(linha.clone());
        }
        Self {
            items: items.clone(),
            matriz: matriz,
        }
        
    }

    fn solve(&mut self, x:usize, y:usize) -> usize{
        let no_atual = self.matriz[y][x].clone();
        if no_atual.visitado {
            return no_atual.valor;
        }

        let peso:i32 = self.items[y-1].weight as i32;
        let adiciona;
        if (x as i32) - peso >=0 {
            adiciona = self.solve(x-(peso as usize), y-1) + self.items[y-1].value;
            
        }else {
            adiciona = 0;
        }
        let recusa = self.solve(x, y-1);

        let maximo = max(adiciona, recusa);
        self.matriz[y][x] = No {
            valor: maximo,
            visitado:true,
        };
        println!("y: {}; x: {}; adicona: {}; recusa: {}; maximo: {}", y, x, adiciona, recusa, maximo);

        maximo
    }

    fn encontrar_caminho(self, x:usize, y:usize, mut retorno:Vec<Item>) -> Vec<Item>{
        if y==0 || x == 0{
            return retorno;
        }
        let item =  self.items[y-1].clone();
        if self.matriz[y][x].valor == self.matriz[y-1][x].valor {
            
            return self.encontrar_caminho(x, y-1, retorno);
        } else{
            retorno.push(item.clone());
            return self.encontrar_caminho(x-(item.weight), y-1, retorno);
        }
    }

}

fn max(a:usize, b:usize) ->usize{
    if a>=b{
        return a
    }

    b
}