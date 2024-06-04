use nalgebra::{DMatrix, DVector};
use std::cmp::min;
pub struct PublicData
{
    msg: Vec<DVector<u128>>,
    //Vector of coefficient matrices for improved cryptographic security (each block has a unique line)
    //this is a direct trade off of memory for speed. 
    coefficients: Vec<DMatrix<u128>>
}

impl PublicData
{   
    /** Creates a new PublicData instance, encrypting the message and consuming both parameters */
    pub fn new (msg_bytes: &mut Vec<u8>, private_keys: &mut Vec<u8>) -> Self
    {        
        let mut msg: Vec<DVector<u128>> = PublicData::fill_msg(msg_bytes);
        let coefficients: Vec<DMatrix<u128>> = PublicData::fill_coefficient_matrices(private_keys);

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

    pub fn decrypt(self) -> String
    {
        let out: String = String::new();
        for i in 0..self.msg.len()
        {
        }

        "".to_string()
    }

    /** Partitions a message into vectors. These will later be multiplied by a matrix to create the rs encoding.
     * 
     * NOTE: This method consumes msg_bytes. The unencoded data should not be needed after this point.
     */
    fn fill_msg (msg_bytes: &mut Vec<u8>) -> Vec<DVector<u128>>
    {
        let mut out: Vec<DVector<u128>> = Vec::new();

        let full_blocks = msg_bytes.len() / 10;
        let partial_len = msg_bytes.len() - 10 * full_blocks;

        for _ in 0..full_blocks
        {
            out.push(DVector::from_iterator(10, msg_bytes.drain(0..10).map(|n| n as u128)));
        }
        out.push(DVector::from_iterator(partial_len, msg_bytes.drain(0..partial_len).map(|n| n as u128)));
        out
    }

    /** Fills all coefficient matrices for this file
     * 
     * NOTE: Consumes private keys to generate this matrix. They should not be needed after this point.
     */
    fn fill_coefficient_matrices (private_keys: &mut Vec<u8>) -> Vec<DMatrix<u128>>
    {
        private_keys.reverse();
        println!("Private Keys: {:?}", private_keys);

        let mut out: Vec<DMatrix<u128>> = Vec::new();

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
    fn fill_matrix (size: usize, private_keys: &mut Vec<u8>) -> DMatrix<u128>
    {
        let mut data: Vec<u128> = Vec::new();
        let mut val: u128;
        for _ in 0..size
        {
            val = private_keys.pop().expect("u8 val") as u128;
            for i in 0..size as u32
            {
                data.push(val.pow(size as u32 - i - 1) as u128);
            }
        }
        
        let mut out = DMatrix::from_vec(size, size, data);
        out.transpose_mut();
        out
    }
}