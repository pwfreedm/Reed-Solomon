use encoding::PublicData;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod interface;
mod encoding;

//ensures that each block has unique random x vals that cannot overflow u128


fn main() {
    let raw_data = interface::get_text_to_encode();
    let private_keys = generate_x_values (raw_data.len()); 
    let encoded = PublicData::new(&mut raw_data.as_bytes().to_vec(), &mut private_keys.clone());
    encoded.print_encoded_message();
    encoded.print_coefficient_matrices();
}

/** Generates the 'private keys' of the encryption. These are x coordinates of points on a line
 * 
 * These x coordinates are bounded in the range [2,12) because the highest power any of them will 
 * be taken to is 9. 11^9 fits into a u32, 12^9 does not. u32 was picked because [1,10) was the minimum
 * desirable range, to prevent x values from being too homogenized, and a u32 was required for that range.
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
    println! ("Out after for loop: {:?}", out);
    println! ("x_vals after for loop: {:?}", x_vals);
    x_vals.shuffle(&mut thread_rng());
    println! ("Length of x_vals: {}", (*x_vals).len());
    out.append(&mut x_vals[0..partial_seg_size].to_vec());

    out
}

