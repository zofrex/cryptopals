mod hex;

#[cfg(not(test))]
fn main() {
    let input = std::old_io::stdin().read_line().ok().expect("Failed to read line");
    let input = input.trim();
    println!("Output: {:?}", hex::hex_to_base64(&input));
}
