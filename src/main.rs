use encoding::PublicData;
use rand::rngs::SmallRng;
use rand::{SeedableRng, Rng};

mod interface;
mod encoding;

fn main() {
    let raw_data = interface::get_text_to_encode();
    let private_keys = generate_x_values (raw_data.len(), 0); 
    let encoded = PublicData::new(&mut raw_data.as_bytes().to_vec(), &mut private_keys.clone());
    encoded.print_encoded_message();
    encoded.print_coefficient_matrices();
}

/** Generates the 'private keys' of the encryption. These are x coordinates of points on a line
 * 
 * These x coordinates are bounded in the range [1,12) because the highest power any of them will 
 * be taken to is 9. 11^9 fits into a u32, 12^9 does not. u32 was picked because [1,10) was the minimum
 * desirable range, to prevent x values from being too homogenized, and a u32 was required for that range.
 */
fn generate_x_values (length: usize, seed: u64) -> Vec<u8>
{
    let mut output = Vec::<u8>::with_capacity(length);
    let mut rng = SmallRng::seed_from_u64(seed);
    for _ in 0..length
    {
        output.push(rng.gen_range(1..12));
    }
    output
}

