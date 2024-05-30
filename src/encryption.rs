use crate::pub_data::PublicData;
//return a coefficient matrix and a vec<vec> of y values
//coefficient matrix is just 
pub fn encrypt (raw_data: &String, private_keys: Vec<i8>)
{
    let mut nums: Vec<u8> = raw_data.as_bytes().to_vec();
    PublicData::blocks(&mut nums, &private_keys);
}