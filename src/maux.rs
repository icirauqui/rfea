
macro_rules! mprint {
    ($matrix:expr) => {
        for i in 0..$matrix.shape().0 {
            for j in 0..$matrix.shape().1 {
                print!("\t{:<3}", $matrix[(i, j)]);
            }
            println!();
        }
    };
}