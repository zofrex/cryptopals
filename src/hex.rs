pub fn hex_to_int(hex: char) -> int {
    if hex >= '0' && hex <= '9' {
        hex as int - '0' as int
    }
    else if hex >= 'a' && hex <= 'f' {
        hex as int - 'a' as int + 10
    }
    else {
        panic!("Character {} out of range", hex);
    }
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

#[test]
fn all_hexes() {
    let hexes = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    for i in range(0u, hexes.len()) {
        assert_eq!(hex_to_int(hexes[i]), i as int);
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
fn look_up_base64_index_caps() {
    let base64s = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    for i in range(0u, base64s.len()) {
        assert_eq!(index_base64(i as u8), base64s[i]);
    }
}

#[test]
fn look_up_base64_index_lowers() {
    let base64s = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    for i in range(0u, base64s.len()) {
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
