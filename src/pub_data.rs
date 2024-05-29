
struct Public_Data
{
    msg: Vec<Vec<i128>>,
    //Vector of coefficient matrices for improved cryptographic security (each block has a unique line)
    //this is a direct trade off of memory for speed. 
    coefficients: Vec<i8>
}