pub fn hex_to_int(hex: char) -> u8 {
    if hex >= '0' && hex <= '9' {
        hex as u8 - '0' as u8
    }
    else if hex >= 'a' && hex <= 'f' {
        hex as u8 - 'a' as u8 + 10
    }
    else {
        panic!("Character {} out of range", hex);
    }
}

fn hex_string_to_int_array(hex: &str) -> Vec<u8> {
    hex.chars().map(hex_to_int).collect()
}

pub fn index_base64(index: u8) -> char {
    if index < 26 {
        ('A' as u8 + index) as char
    }
    else if index < 52 {
        ('a' as u8 + index - 26) as char
    }
    else if index == 62 {
        '+'
    }
    else if index == 63 {
        '/'
    }
    else {
        panic!("Index {} out of range", index);
    }
}

pub fn int_triplet_to_base64(triplet: &[u8; 3]) -> Vec<char> {
    let mut indexes: [u8; 4] = [0,0,0,0];
    indexes[0] = (triplet[0] & 0b11111100) >> 2;
    indexes[1] = ((triplet[0] & 0b00000011) << 4) | ((triplet[1] & 0b11110000) >> 4);
    indexes[2] = ((triplet[1] & 0b00001111) << 2) | ((triplet[2] & 0b11000000) >> 6);
    indexes[3] = triplet[2] & 0b00111111;
    indexes.iter().map(|i| index_base64(*i)).collect()
}

pub fn int_couplet_to_base64(couplet: &[u8; 2]) -> Vec<char> {
    let mut result = int_triplet_to_base64(&[couplet[0], couplet[1], 0]);
    result[3] = '=';
    result
}

pub fn int_singlet_to_base64(singlet: &[u8; 1]) -> Vec<char> {
    let mut result = int_couplet_to_base64(&[singlet[0], 0]);
    result[2] = '=';
    result
}

#[test]
fn all_hexes() {
    let hexes = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    for i in 0us..hexes.len() {
        assert_eq!(hex_to_int(hexes[i]), i as u8);
    }
}

#[test]
#[should_fail]
fn test_too_large() {
    hex_to_int('g');
}

#[test]
#[should_fail]
fn test_too_small() {
    hex_to_int('/');
}

#[test]
fn test_string_to_int() {
    assert_eq!(hex_string_to_int_array("a"), vec![10]);
    assert_eq!(hex_string_to_int_array("05af"), vec![0, 5, 10, 15]);
}

#[test]
fn look_up_base64_index_caps() {
    let base64s = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    for i in 0us..base64s.len() {
        assert_eq!(index_base64(i as u8), base64s[i]);
    }
}

#[test]
fn look_up_base64_index_lowers() {
    let base64s = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    for i in 0us..base64s.len() {
        assert_eq!(index_base64(i as u8 + 26), base64s[i]);
    }
}

#[test]
fn look_up_base64_others() {
    assert_eq!(index_base64(62), '+');
    assert_eq!(index_base64(63), '/');
}

#[test]
#[should_fail]
fn look_up_base64_too_large() {
    index_base64(64);
}

#[test]
fn test_triplet_to_base64() {
    assert_eq!(int_triplet_to_base64(&[77, 97, 110]), vec!['T', 'W', 'F', 'u']);
}

#[test]
fn test_couplet_to_base64() {
    assert_eq!(int_couplet_to_base64(&[77, 97]), vec!['T', 'W', 'E', '=']);
}

#[test]
fn test_singlet_to_base64() {
    assert_eq!(int_singlet_to_base64(&[77]), vec!['T', 'Q', '=', '=']);
}
