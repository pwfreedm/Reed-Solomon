use rand::rngs::{SmallRng};
use rand::{SeedableRng, Rng};

mod interface;
mod encryption;
fn main() {
    let raw_data = interface::get_text_to_encode();
    let private_keys = generate_x_values (raw_data.len(), 0); 
}

/** Generates the 'private keys' of the encryption. These are x coordinates of points on a line
 * 
 * Private key values are bounded by i8 for ease of manual checking.
 */
fn generate_x_values (length: usize, seed: u64) -> Vec<i8>
{
    let mut output = Vec::<i8>::with_capacity(length);
    let mut rng = SmallRng::seed_from_u64(seed);
    for _ in 0..length
    {
        output.push(rng.gen_range(0..i8::MAX));
    }
    output
}
