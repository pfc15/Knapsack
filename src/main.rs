use std::time::Instant;
mod knapsack;
mod sequence_allingment;
use knapsack::Item;
use knapsack::Knapsack;
use sequence_allingment::Sequence;


fn main() {
    chama_sequence_alling();
}

fn chama_knapsack() {
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

fn chama_sequence_alling() {
    let mut mismatch:Vec<Vec<usize>> = Vec::new();
    let s1 = "ctgacctacct";
    let s2 = "cctgactacat";

    for _y in 0..s1.len(){
        let mut linha:Vec<usize> = Vec::new();
        for _x in 0..s2.len() {
            linha.push(1);
        }
        mismatch.push(linha.clone());
    }

    let mut obj_sequencia = Sequence::new(s1,s2, mismatch);

    obj_sequencia.populate_matriz(0, 0);
    
    println!("{}", obj_sequencia.s2);
    println!("{}", obj_sequencia.s1);

    for linha in obj_sequencia.matriz {
        for v in linha {
            print!("{} | ", v);
        }
        println!("");
    }
}
