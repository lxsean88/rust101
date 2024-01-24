use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "( {} {} )\n( {} {} )",
                self.0, self.1,
                self.2, self.3)
     }
}


fn transpose(m: Matrix) -> Matrix {
     Matrix(m.0, m.2, m.1, m.3)
}

fn main() {
     // Tuples can be tuple members.
     let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

     // Tuples are printable.
     println!("tuple of tuples: {:?}", tuple_of_tuples);
     println!("tuple of tuples: {:#?}", tuple_of_tuples);


    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

}