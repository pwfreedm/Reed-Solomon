use pub_data::PublicData;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod interface;
mod pub_data;

fn main() {
    let raw_data = interface::get_text_to_encode();
    let private_keys = generate_x_values (raw_data.len()); 
    let encoded = PublicData::new(&mut raw_data.as_bytes().to_vec(), &mut private_keys.clone());
    encoded.print_encoded_message();
    encoded.print_coefficient_matrices();
    println! ("{}", encoded.decrypt());
}

/** Generates the 'private keys' of the encryption. These are x coordinates of points on a line
 * 
 * These x coordinates are bounded in the range [2,12) because the highest power any of them will 
 * be taken to is 9, and going above 12 risks overflowing when solving the matrix later.
 * 
 * The reasoning behind using the vector is to insure that each block of message has a unique solution.
 */
fn generate_x_values (length: usize) -> Vec<u8>
{
    let x_vals = &mut Vec::from_iter(2..12);

    let full_seg = length / 10;
    let partial_seg_size = length % 10;

    let mut out: Vec<u8> = Vec::with_capacity(length);
    for _ in 0..full_seg
    {
        x_vals.shuffle(&mut thread_rng());
        out.append(&mut x_vals.clone());
    }
    x_vals.shuffle(&mut thread_rng());
    out.append(&mut x_vals[0..partial_seg_size].to_vec());

    out
}

