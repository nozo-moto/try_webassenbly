extern crate num;
//extern crate ndarray;
extern crate rand;
extern crate ndarray_rand;
use rand::distributions::*;
use ndarray_rand::RandomExt;

fn main() {
    let mut rng = rand::thread_rng();
    // let mut mat1 = [[1, ..1000], ..1000];
    // let mut mat2 = [[1, ..1000], ..1000];
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let r_dist = Range::new(0., 10.);
    let mut mat1 = Array::<f64, _>::random((1000, 1000), r_dist);
    let mut mat2 = Array::<f64, _>::random((1000, 1000), r_dist);
    for i in 0..1000{
        for j in 0..1000{ 
              for k in 0..1000{ 
                  println!("i:{} j:{} k:{}", i, j, k);
              }
          }
      }
}
