use nalgebra::{DMatrix, DVector};
use std::{cmp::min, str::from_utf8};
pub struct PublicData
{
    msg: Vec<DVector<f64>>,
    //Vector of coefficient matrices for improved cryptographic security (each block has a unique line)
    //this is a direct trade off of memory for speed. 
    coefficients: Vec<DMatrix<f64>>
}

impl PublicData
{   
    /** Creates a new PublicData instance, encrypting the message and consuming both parameters */
    pub fn encrypt (msg_bytes: &mut Vec<u8>, private_keys: &mut Vec<u8>) -> Self
    {        
        let mut msg: Vec<DVector<f64>> = PublicData::fill_msg(msg_bytes);
        let coefficients: Vec<DMatrix<f64>> = PublicData::fill_coefficient_matrices(private_keys);

        //this is actually encoding the message data
        for i in 0..msg.len()
        {
            //the clones are tragic but the compiler would copy them out anyways, so it doesn't change much alloc wise
            msg[i] = coefficients[i].clone() * msg[i].clone();
        }
        Self {msg: msg, coefficients: coefficients}
    }

    pub fn print_coefficient_matrices (&self)
    {
        println!("******************************************************************************************");
        println!("Coefficient Matrices: ");
        for v in 0..self.coefficients.len()
        {
            println!("{:#}", self.coefficients[v]);
        }
        println!("******************************************************************************************");
    }
    
    pub fn print_encoded_message (&self)
    {
        println!("******************************************************************************************");
        println!("Encoded Message: ");
        for v in 0..self.msg.len()
        {
            println!("{:#}", self.msg[v]);
        }
        println!("******************************************************************************************");    
    }

    /** Decrypts this instance of public data using the encoded message and the coefficient matrices
     * 
     * The coefficient matrices are effectively the private keys, so in non-educational settings those would 
     * not be publicly accessible, but instead agreed upon by sender and receiver by other means. 
     * 
     * The reason round() is used is because nalgebra's matrix solving methods both only work on fp type matrices
     * and introduce fp rounding errors that throw off the utf-8 decoding
     */
    pub fn decrypt(self) -> String
    {
        let mut out: String = String::new();
        for i in 0..self.msg.len()
        {
            let sln: &mut DVector<f64> = &mut self.msg[i].clone();
            self.coefficients[i].clone().lu().solve_mut(sln);
            let chars: Vec<u8> = sln.data.as_vec().into_iter().map(|n| n.round() as u8).collect();
            out.push_str(from_utf8(&chars).expect("utf-8 string"));
        }

        out
    }

    /** Partitions a message into vectors. These will later be multiplied by a matrix to create the rs encoding.
     * 
     * NOTE: This method consumes msg_bytes. The unencoded data should not be needed after this point.
     */
    fn fill_msg (msg_bytes: &mut Vec<u8>) -> Vec<DVector<f64>>
    {
        let mut out: Vec<DVector<f64>> = Vec::new();

        let full_blocks = msg_bytes.len() / 10;
        let partial_len = msg_bytes.len() - 10 * full_blocks;

        for _ in 0..full_blocks
        {
            out.push(DVector::from_iterator(10, msg_bytes.drain(0..10).map(|n| n as f64)));
        }
        out.push(DVector::from_iterator(partial_len, msg_bytes.drain(0..partial_len).map(|n| n as f64)));
        out
    }

    /** Fills all coefficient matrices for this file
     * 
     * NOTE: Consumes private keys to generate this matrix. They should not be needed after this point.
     */
    fn fill_coefficient_matrices (private_keys: &mut Vec<u8>) -> Vec<DMatrix<f64>>
    {
        private_keys.reverse();

        let mut out: Vec<DMatrix<f64>> = Vec::new();

        let mut len = private_keys.len();

        while len > 0
        {

            out.push(PublicData::fill_matrix(min(10, len), private_keys));
            len -= min(10, len);
        }
        out
    }

    /** Fills a single coefficient matrix given a size.
     * 
     * NOTE: Consumes private keys to generate this matrix
     */
    fn fill_matrix (size: usize, private_keys: &mut Vec<u8>) -> DMatrix<f64>
    {
        let mut data: Vec<f64> = Vec::new();
        let mut val: f64;
        for _ in 0..size
        {
            val = private_keys.pop().expect("u8 val") as f64;
            for i in 0..size
            {
                data.push(f64::powi(val, (size - i - 1) as i32) as f64);
            }
        }
        
        let mut out = DMatrix::from_vec(size, size, data);
        out.transpose_mut();
        out
    }
}