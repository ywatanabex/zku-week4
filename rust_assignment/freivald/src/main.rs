use rand::random;
use ndarray::{Array1, Array2};
use ndarray::array;
use ark_ff::field_new;
use ark_bls12_381::Fq;


struct Freivald {
    x: Array1<Fq>,
}

impl Freivald {
    fn new(array_size: usize) -> Self {
        let r = random::<Fq>();
        let mut _r = r;
        let mut x = Array1::<Fq>::zeros((array_size,));
        for n in 0..array_size {
            x[n] = _r;
            _r = _r * r;
        }
        Self { x } 
    }

    fn verify(&self, matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        if !check_matrix_dimensions(matrix_a, matrix_b, supposed_ab) {return false};
        let bx = matrix_b.dot(&self.x);
        let abx = matrix_a.dot(&bx);
        let cx = supposed_ab.dot(&self.x);
        return abx == cx;
    }

    fn verify_once(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}
// TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)


// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    todo!()
}

pub fn check_matrix_dimensions(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
    let c0 = match (matrix_a.shape().get(0), supposed_ab.shape().get(0)) {(Some(x), Some(y)) => x == y, _ => false};
    let c1 = match (matrix_a.shape().get(1), matrix_b.shape().get(0)) {(Some(x), Some(y)) => x == y, _ => false};
    let c2 = match (matrix_b.shape().get(1), supposed_ab.shape().get(1)) {(Some(x), Some(y)) => x == y, _ => false};
    return c0 && c1 && c2;
}


#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;

    lazy_static! {
        //todo!("add matrices types and values")
        static ref MATRIX_A: Array2<Fq> = array![[field_new!(Fq, "1"), field_new!(Fq, "1")], [field_new!(Fq, "0"), field_new!(Fq, "1")]];
        static ref MATRIX_A_DOT_A: Array2<Fq> = array![[field_new!(Fq, "1"), field_new!(Fq, "2")], [field_new!(Fq, "0"), field_new!(Fq, "1")]];
        static ref MATRIX_B: Array2<Fq> = array![[field_new!(Fq, "3"), field_new!(Fq, "0"), field_new!(Fq, "-1")], [field_new!(Fq, "1"), field_new!(Fq, "5"), field_new!(Fq, "2")], [field_new!(Fq, "-1"), field_new!(Fq, "6"), field_new!(Fq, "2")]];        
        static ref MATRIX_B_DOT_B: Array2<Fq> = array![[field_new!(Fq, "3"), field_new!(Fq, "0"), field_new!(Fq, "-1")], [field_new!(Fq, "1"), field_new!(Fq, "5"), field_new!(Fq, "2")], [field_new!(Fq, "-1"), field_new!(Fq, "6"), field_new!(Fq, "2")]].dot(&array![[field_new!(Fq, "3"), field_new!(Fq, "0"), field_new!(Fq, "-1")], [field_new!(Fq, "1"), field_new!(Fq, "5"), field_new!(Fq, "2")], [field_new!(Fq, "-1"), field_new!(Fq, "6"), field_new!(Fq, "2")]]); 
        // TODO: find a way to initialize a randam matrix in lazy_static
        static ref MATRIX_C: Array2<Fq> = Array2::<Fq>::zeros((200, 200)); 
        static ref MATRIX_C_DOT_C: Array2<Fq> = Array2::<Fq>::zeros((200, 200)); 
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Array2<Fq>,
        #[case] matrix_b: &Array2<Fq>,
        #[case] supposed_ab: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &Array2<Fq>,
        #[case] b: &Array2<Fq>,
        #[case] c: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
}
