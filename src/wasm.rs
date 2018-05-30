#[no_mangle]
pub fn sum() -> i32 {

    let mut _i = 0;
    let mut _j = 0;
    let mut _k = 0;
    let mut rel = 0;
    let mut mat1 = [[0;100];100];
    let mut mat2 = [[0;100];100];
    let mut mat3 = [[0;100];100];
    for _i in 0..99 {
        for _j in 0..99 {
            mat1[_i][_j] = 5;
            mat2[_i][_j] = 7;
        }
    }
    for _i in 0..99{
        for _j in 0..99{
            for _k in 0..99{
                mat3[_i][_j] = rel + mat1[_i][_k] * mat2[_k][_j];
            }
        }
    }
    return 0;
}
