extern crate num;
extern crate rand;
use rand::Rng;

fn main() {
    let mut _i = 0;
    let mut _j = 0;
    let mut _k = 0;
    let mut rng = rand::thread_rng();
    let mut mat1 = [[0;1000];1000];
    let mut mat2 = [[0;1000];1000];
    let mut mat3 = [[0;1000];1000];
    let mut rel = 0;
    for _i in 0..1000 {
        for _j in 0..1000 {
            mat1[_i][_j] = rng.gen_range(0, 10);
            mat2[_i][_j] = rng.gen_range(0, 10);
            // result[_i][_j] = 0;
        }
    }

    // for _i in 0..1000 {
    //     for _j in 0..1000 {
    //         println!("{} ", mat1[_i][_j]);
    //     }
    //     print!("\n");
    // }
    // let hoge = [0;1000];
    println!("len mat1 {}", mat1.len());
    for _i in 0..999{
        for _j in 0..999{ 
            rel = 0;
            for _k in 0..999{
                rel = rel + mat1[_i][_k] * mat2[_k][_j];
                // rlt[_i][_j] = rlt[_i][_j] + mat1[_i][_k] * mat2[_k][_j];
            }
            // println!("result = {}", rel);
        }
    }
}
