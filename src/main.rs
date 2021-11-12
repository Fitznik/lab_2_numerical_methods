use std::fmt;

fn main() {

    let matrix = [ 
        [2.0, 4.0, 0.0],
        [4.0, 1.0, 5.0],
        [0.0, 5.0, 2.0]
    ];

    let matrix1 = Matrix::<3, 3>{
        data: matrix
    };

    const N: usize = 3;
    const M: usize = 3;

    println!("{}", signum(-7.0));
    println!("{}", matrix1);
    let exam_matr= Matrix::<3,3>::zero();
    println!("{}", exam_matr)
    // println!("{:?}", matrix1.shape());
    // println!("{:?}", matrix1.ncols());
    // println!("{:?}", matrix1.nrows());

}

fn signum(x: f64)->f64{
    if x< 0.0{
        -1.0
    } else if x==0.0{
        0.0
    } else {
        1.0
    }
}

struct Matrix< const N: usize, const M: usize>{
    data:  [[f64; N]; M]
}

impl<const N: usize, const M: usize> Matrix<N, M>{

    fn zero() -> Self {
        Matrix{ data: [[0f64; N]; M] }
    }

    fn add(&self, other: &Self) -> Self{
        let mut m_res = Self::zero();
        for i in 0..N {
            for j in 0..M {
                m_res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        
        m_res
    }
    #[inline]
    #[must_use]
    fn shape(&self) -> (usize, usize) {
        let (nrows, ncols) = (M, N);
        (nrows, ncols)
    }

    #[inline]
    #[must_use]
    fn nrows(&self) -> usize {
        self.shape().0
    }
    #[inline]
    #[must_use]
    fn ncols(&self) -> usize {
        self.shape().1
    }


    // square root method

    // fn square_root_method(&self)-> Self{
    //     exam_matr= Self::zero();

    //     for i in 0..N+1{


    //     }

    // }

}
use std::fmt::Display;
impl<const N: usize, const M: usize> fmt::Display for Matrix<N,M>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let n_row = self.nrows();
        let n_cols = self.ncols();
        let data = self.data;



        for i in 0.. n_row {
            write!(f, "  │")?;
            let mut each_row = String::new(); 

            for j in 0.. n_cols {
                let mut idx = i ;  
                let mut idy = j ;
                let mut each_element = data[idx][idy];
                each_row.push_str(&each_element.to_string());
                each_row.push_str(" "); 
            }
         r#try!(write!(f, "{:width$}│ \n", each_row,  width = 10));
        }
         
        
        Ok(())}
}
 