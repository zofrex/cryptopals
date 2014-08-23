mod hex;

#[cfg(not(test))]
fn main() {
    println!("Output: {}", hex::hex_to_int('0'));
}
