use nalgebra::{DMatrix, DVector};
pub struct PublicData
{
    msg: Vec<DVector<u128>>,
    //Vector of coefficient matrices for improved cryptographic security (each block has a unique line)
    //this is a direct trade off of memory for speed. 
    coefficients: Vec<DMatrix<u32>>
}

//1) populate the msg vectors
//2) calculate coefficient matrices
//3) overwrite msg vectors with encoded values
impl PublicData
{
    pub fn new (msg_bytes: &mut Vec<u8>, private_keys: &Vec<i8>) -> Self
    {        
        let msg: Vec<DVector<u128>> = PublicData::fill_msg(msg_bytes);
        
    }

    /** Partitions a message into vectors. These will later be multiplied by a matrix to create the rs encoding.
     * 
     * NOTE: This method consumes msg_bytes using drain. The unencoded data should not be needed after this point.
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
}