fn main() {
    let mut text = String::from("Пик Балмера");
    reverse_words_at_odd_positions(&mut text);
    println!("{text}");
}

fn reverse_words_at_odd_positions(text: &mut String) {
    let bytes = unsafe { text.as_bytes_mut() };
    let mut pos = 0;
    let mut index = 0;
    while pos < bytes.len() {
        let c = decode_char(bytes, pos);
        if c.is_whitespace() {
            pos += c.len_utf8();
            continue;
        }
        let start = pos;
        while pos < bytes.len() {
            let c = decode_char(bytes, pos);
            if c.is_whitespace() {
                break;
            }
            pos += c.len_utf8();
        }
        if index % 2 == 0 {
            reverse_word(&mut bytes[start..pos]);
        }
        index += 1;
    }
}

fn decode_char(bytes: &[u8], pos: usize) -> char {
    let len = match bytes[pos] {
        b if b < 0x80 => 1,
        b if b >= 0xF0 => 4,
        b if b >= 0xE0 => 3,
        _ => 2,
    };
    std::str::from_utf8(&bytes[pos..pos + len])
        .unwrap()
        .chars()
        .next()
        .unwrap()
}

fn reverse_word(word: &mut [u8]) {
    word.reverse();
    let mut end = 0;
    while end < word.len() {
        let start = end;
        end += 1;
        while end < word.len() && word[end - 1] & 0xC0 == 0x80 {
            end += 1;
        }
        word[start..end].reverse();
    }
}

#[cfg(test)]
#[path = "../tests/unit/monologue.rs"]
mod monologue_tests;
