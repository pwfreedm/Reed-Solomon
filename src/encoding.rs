use nalgebra::{DMatrix, DVector};
pub struct PublicData
{
    msg: Vec<DVector<u128>>,
    //Vector of coefficient matrices for improved cryptographic security (each block has a unique line)
    //this is a direct trade off of memory for speed. 
    coefficients: Vec<DMatrix<u128>>
}

impl PublicData
{
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

    fn fill_coefficient_matrices (private_keys: &mut Vec<u8>) -> Vec<DMatrix<u128>>
    {
        println!("Private keys: {:?}", private_keys);
        private_keys.reverse();
        let mut out: Vec<DMatrix<u128>> = Vec::new();

        let full_blocks = private_keys.len() / 10;
        let partial_len = private_keys.len() - 10 * full_blocks;

        let mut val = private_keys.pop().expect("u8 value") as u128;
        let mut acc: Vec<u128> = Vec::new();

        for _ in 0..full_blocks
        {

            for i in 0..10
            {
                acc.push(val.pow(9 - i) as u128);
            }
           out.push(DMatrix::from_vec(10,10, acc.clone()));
           val = private_keys.pop().expect("u8 value") as u128;
           acc.clear();
        }

        //tiny block
        println!("Current Key: {}", val);
        for _ in 0..partial_len
        {
            for i in 0..partial_len as u32
            {
                acc.push(val.pow((partial_len as u32 - i).try_into().expect("u32")) as u128);
            } 
            val = private_keys.pop().expect("u8 val") as u128;   
            println!("Current Key: {}", val);
        }
        out.push(DMatrix::from_vec(partial_len, partial_len, acc));    


        println!("***************************************************************************");
        println!("{:?}", out);
        println!("***************************************************************************");
        out
    }
}