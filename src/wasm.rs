#[no_mangle]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn it_works() {
	assert!(false, "result is {}", sum(100));
}

#[test]
fn it_works2() {
  assert!(false, "always false");
}

