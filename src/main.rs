use std::fmt;
use round::{round_up, round_down};
use std::cmp::max;

fn main() {

    let matrix1 = [ 
        [2.0, 4.0, 0.0],
        [4.0, 1.0, 5.0],
        [0.0, 5.0, 2.0]
    ];

    let matrix2 = [
        [18.0],
        [33.0],
        [30.0]
    ];

    let matrix3 = [ 
        [3.0, 1.0, 1.0, 0.0],
        [1.0, 4.0, 0.0, 2.0],
        [1.0, 0.0, 3.0, 1.0],
        [0.0, 2.0, 1.0, 5.0]
    ];
    let matrix4 = [
        [8.0],
        [17.0],
        [14.0],
        [27.0]
    ];
    let matrix5 = [ 
        [4.0, 0.0, 1.0, 0.0],
        [0.0, 3.0, 0.0, 2.0],
        [1.0, 0.0, 5.0, 1.0],
        [0.0, 2.0, 1.0, 4.0]
    ];


    let matrix1 = Matrix::<3, 3>{
        data: matrix1
    };
    let matrix2 = Matrix::<1, 3>{
        data: matrix2
    };

    let matrix3 = Matrix::<4, 4>{
        data: matrix3
    };
    let matrix4 = Matrix::<1, 4>{
        data: matrix4
    };
    let matrix5 = Matrix::<4, 4>{
        data: matrix5
    };
    //task 2
    println!("matrix = \n{}, vector = \n{}", matrix1, matrix2 );
    let matrix_sol_y =  Matrix::<3,3>::square_root_method(&matrix1);
    println!("Y solution \n{}", matrix_sol_y.liniar_comb_y(&matrix2));
    let matrix_sol_x = Matrix::<3,3>::s_matrix(&matrix1);
    println!("X solution \n{}",  matrix_sol_x.liniar_comb_x(&matrix_sol_y.liniar_comb_y(&matrix2)));

    // task 3
    println!("{}", matrix3.iter_method(&matrix4));

    // task 4
    println!("Determinant {}", matrix5.determinant());
    let m = lin_zendey();



}

fn lin_zendey(){
    let mut x1 = 0.0;
    let mut x2 = 0.0;
    let mut x3 = 0.0;
    let mut x4 = 0.0;
    for i in 0..11{
        x1 = (7.0 - x2 * 0.0 - x3 * 1.0 -x4 * 0.0)/4.0;
        x2 = (14.0 - x1 * 0.0 - x3 * 0.0 - x4 *2.0)/3.0;
        x3 = (20.0 - x1 * 1.0 - x2 * 0.0 - x4 * 1.0)/5.0;
        x4 = (23.0 -x1 * 0.0- x2 * 2.0 - x3 * 1.0 )/4.0;
        println!("Iteration: {} \nx1 = {}, x2 = {}, x3=  {}, x4 =  {}", i, x1, x2, x3, x4);
    }

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
fn determinant(m: &Vec<Vec<f64>>) -> f64 {
    if m.len() == 0 {
        1f64
    } else {
        let size = m.len();
        let mut sign = 1f64;
        let mut det = 0f64;
        for i in 0..size {
            let minor = determinant(&submatrix(&m, i, 0));
            det += minor * sign * m[i][0];
            sign *= -1f64;
        }
        
        det
    }
}
fn submatrix(m: &Vec<Vec<f64>>, a: usize, b: usize) -> Vec<Vec<f64>> {
    let size = m.len() - 1;

    let mut result: Vec<Vec<f64>> = vec![];
    for i in 0..size {
        let mut sub_res: Vec<f64> = vec![];
        for j in 0..size{
            if i < a {
                if j < b {
                    sub_res.push(m[i][j]);
                } else {
                    sub_res.push(m[i][j+1]);
                }
            } else {
                if j < b {
                    sub_res.push(m[i+1][j]);
                } else {
                    sub_res.push(m[i+1][j+1]);
                }
            }
        }
        result.push(sub_res);
    }
    result
}

impl<const N: usize> Matrix<N, N>{
    
    fn determinant(&self) -> f64 {
        let mut matrix: Vec<Vec<f64>> = vec![];

        for i in 0..N{
            matrix.push(self.data[i].to_vec());
        }

        determinant(&matrix)
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
    fn sub(&self, other: &Self) -> Self{
        let mut m_res = Self::zero();
        for i in 0..N {
            for j in 0..M {
                m_res.data[i][j] = self.data[i][j] - other.data[i][j];
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
  
    fn mul<const K: usize>(&self, other: &Matrix<K,N>) -> Matrix<K,M>{
        let mut c = Matrix::<K,M>::zero();
        for i in 0..M{
            for j in 0..K{
                for n in 0..N{
                    c.data[i][j]+= self.data[i][n]*other.data[n][j];
                }
            }
        }
        return c;
    }

    fn transpose(&self)-> Matrix::<M,N>{
        let mut b= Matrix::<M,N>::zero();
        for i in 0..N{
            for j in 0..M{
                b.data[i][j] = self.data[j][i]
            }

        }
        return b;
    }

    fn liniar_comb_y<const K: usize>(&self, other: &Matrix::<K,N>)->Matrix::<K,N>{
        let mut y= Matrix::<K,N>::zero();
        y.data[0][0]= other.data[0][0]/self.data[0][0];
        y.data[1][0]= (other.data[1][0]-y.data[0][0]*self.data[1][0])/self.data[1][1];
        y.data[2][0]=(other.data[2][0]- y.data[0][0]*self.data[2][0]-y.data[1][0]*self.data[2][1])/self.data[2][2];
        return y;

    }

    fn liniar_comb_x<const K: usize>(&self, other: &Matrix::<K,N>)->Matrix::<K,N>{
        let mut x= Matrix::<K,N>::zero();
        x.data[2][0]= other.data[2][0]/self.data[2][2];
        x.data[1][0]= round_up((other.data[1][0]-x.data[0][0]*self.data[1][0]-x.data[2][0]*self.data[1][2])/self.data[1][1],0);
        x.data[0][0]=round_down((other.data[0][0]- x.data[2][0]*self.data[0][2]-x.data[1][0]*self.data[0][1])/self.data[0][0],0);
        return x;

    }


    // square root method

    fn s_matrix(&self) -> Matrix::<N,M>{
        let mut s= Matrix::<N,M>::zero();
        let mut d= Matrix::<N,M>::zero();



        for i in 0..N{
            let mut dzeta = 0.0;
            for n in 0..i{
                dzeta += (libm::fabs(s.data[n][i]*s.data[n][i]) * d.data[n][n]) as f64;
            }
            d.data[i][i] = signum(self.data[i][i] - dzeta);
            s.data[i][i] = libm::sqrt(libm::fabs(self.data[i][i] - dzeta));


            for j in i..N{
                let mut dzeta_2 = 0.0;
                for n in 0..i{
                    dzeta_2 +=(s.data[n][i] * d.data[n][n]*s.data[n][j]) as f64;

                }
                s.data[i][j]= (self.data[i][j]-dzeta_2)/(d.data[i][i]*s.data[i][i]);
            }
        }
        return s;
    }


    fn square_root_method(&self) -> Matrix::<N,N>{
        assert_eq!(N, M);
        let mut d= Matrix::<N,M>::zero();
        let mut s= Matrix::<N,M>::zero();


        for i in 0..N{
            let mut dzeta = 0.0;
            for n in 0..i{
                dzeta += ((s.data[n][i]*s.data[n][i]) * d.data[n][n]) as f64;
            }
            d.data[i][i] = signum(self.data[i][i] - dzeta);
            s.data[i][i] = libm::sqrt(libm::fabs(self.data[i][i] - dzeta));


            for j in i..N{
                let mut dzeta_2 = 0.0;
                for n in 0..i{
                    dzeta_2 +=(s.data[n][i] * d.data[n][n]*s.data[n][j]) as f64;
                }
                s.data[i][j]= (self.data[i][j]-dzeta_2)/(d.data[i][i]*s.data[i][i]);
            }
        }
        println!("matrix S = \n{}, matrix D = \n{}", s, d);
        let s_transpose = s.transpose();
        let st_d = s_transpose.mul(&d);
        println!("S^t*D = \n{}", st_d);
        return st_d;

        
    }


    //iterators method
    fn iter_method(&self, other: &Matrix::<1,M>)-> Matrix::<1,M>{
        let mut d= Matrix::<1,M>::zero();
        let mut vec = Vec::new();
        let mut summa = 0.0;
        for i in 0..M{
            summa = 0.0;
            for j in 0..N{
                summa += self.data[j][i];
            } 
            vec.push(summa);
        }
        let v_max = vec.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        println!("max elemant: {}", v_max);
        let tau = 2.0/v_max;
        println!("Tau is : {}", tau);

        for i in 0..M{
            d.data[i][0] += tau * other.data[i][0];
        }


        return d;
    }

}

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
         r#try!(write!(f, "{:width$}│ \n", each_row,  width = 2));
        }
         
        
        Ok(())}
}
 