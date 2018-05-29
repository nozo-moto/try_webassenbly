extern crate rand;
extern crate num;
use rand::Rng;
#[no_mangle]
pub fn sum(a: i32, b: i32) -> i32 {
  let mut rng = rand::thread_rng();
  let mut mat1 = [[0, ..1000], ..1000];
      let mut mat2 = [[1, ..1000], ..1000];
      let mut i = 0;
      let mut j = 0;
      let mut k = 0;
      for i in 0..1000{
          for j in 0..1000{
              mat1[i][j] = num::abs(rng.gen::<i32>() % 10);
              mat2[i][j] = num::abs(rng.gen::<i32>() % 10);
          }
      }
      for i in 0..1000{ 
          for j in 0..1000{ 
              for k in 0..1000{ 
                  println!("i:{} j:{} k:{}", i, j, k);
              }
          }
      }
}
