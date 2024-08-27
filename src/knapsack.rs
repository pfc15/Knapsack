
#[derive(Clone)]
pub struct Item {
    pub value: usize,
    pub weight: usize
}

#[derive(Clone)]
pub struct No {
    pub valor: usize,
    pub visitado: bool
}

#[derive(Clone)]
pub struct Knapsack {
    pub items: Vec<Item>,
    pub matriz: Vec<Vec<No>>,
}

impl Knapsack {
    pub fn new(items:Vec<Item>) -> Self{
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

    pub fn solve(&mut self, x:usize, y:usize) -> usize{
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

    pub fn encontrar_caminho(self, x:usize, y:usize, mut retorno:Vec<Item>) -> Vec<Item>{
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