pub fn encode_b32(i: &u8) -> char {
    if i == &(0u8) {
        return '0';
    } else if i == &(1) {
        return '1';
    } else if i == &(2) {
        return '2';
    } else if i == &(3) {
        return '3';
    } else if i == &(4) {
        return '4';
    } else if i == &(5) {
        return '5';
    } else if i == &(6) {
        return '6';
    } else if i == &(7) {
        return '7';
    } else if i == &(8) {
        return '8';
    } else if i == &(9) {
        return '9';
    } else if i == &(10) {
        return 'b';
    } else if i == &(11) {
        return 'c';
    } else if i == &(12) {
        return 'd';
    } else if i == &(13) {
        return 'e';
    } else if i == &(14) {
        return 'f';
    } else if i == &(15) {
        return 'g';
    } else if i == &(16) {
        return 'h';
    } else if i == &(17) {
        return 'j';
    } else if i == &(18) {
        return 'k';
    } else if i == &(19) {
        return 'm';
    } else if i == &(20) {
        return 'n';
    } else if i == &(21) {
        return 'p';
    } else if i == &(22) {
        return 'q';
    } else if i == &(23) {
        return 'r';
    } else if i == &(24) {
        return 's';
    } else if i == &(25) {
        return 't';
    } else if i == &(26) {
        return 'u';
    } else if i == &(27) {
        return 'v';
    } else if i == &(28) {
        return 'w';
    } else if i == &(29) {
        return 'x';
    } else if i == &(30) {
        return 'y';
    } else if i == &(31) {
        return 'z';
    } else {
        return 'a';
    }
}