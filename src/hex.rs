pub fn hex_to_dec(hex: char) -> int {
    if hex >= '0' && hex <= '9' {
        hex as int - '0' as int
    }
    else if hex >= 'a' && hex <= 'f' {
        hex as int - 'a' as int + 10
    }
    else {
        fail!("Character {} out of range", hex);
    }
}

#[test]
fn all_hexes() {
    let hexes = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    for i in range(0u, hexes.len()) {
        assert_eq!(hex_to_dec(hexes[i]), i as int);
    }
}

#[test]
#[should_fail]
fn test_too_large() {
    hex_to_dec('g');
}

#[test]
#[should_fail]
fn test_too_small() {
    hex_to_dec('/');
}