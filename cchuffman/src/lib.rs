use std::collections::HashMap;

use huffman::HuffmanTree;
mod huffman;

pub fn encode(text: &str) -> Vec<u8> {
    let tree = HuffmanTree::from(text);
    let table = tree.build_table();
    let mut capacity = tree.byte_size();
    for (_, prefix) in table.iter() {
        capacity += 4 + 1 + prefix.as_bytes().len() + 1;
    }
    capacity += 16;
    let mut buffer = Vec::with_capacity(capacity);
    let table_encoded = encode_table(&table);
    let header_len = (table_encoded.len()).to_be_bytes();
    buffer.extend(header_len);
    buffer.extend(table_encoded);
    let text_encoded = encode_text(&table, text);
    let body_len = (text_encoded.len()).to_be_bytes();
    buffer.extend(body_len);
    buffer.extend(text_encoded);
    buffer
}

fn encode_table(table: &HashMap<char, String>) -> Vec<u8> {
    let mut buffer = Vec::new();
    for (character, prefix) in table.iter() {
        buffer.extend_from_slice((character.to_string()).as_bytes());
        buffer.push(b':');
        buffer.extend_from_slice(prefix.as_bytes());
        buffer.push(b',');
    }
    buffer
}
fn encode_text(table: &HashMap<char, String>, text: &str) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut byte: u8 = 0;
    let mut i = 7;
    for character in text.chars() {
        let prefix = table.get(&character).unwrap();
        for bit in prefix.chars() {
            let mut bit = if bit == '1' { 1 } else { 0 };
            bit = bit << i;
            byte |= bit;
            i -= 1;
            if i < 0 {
                buffer.push(byte);
                byte = 0;
                i = 7;
            }
        }
    }
    buffer.push(byte);
    buffer
}

pub fn decode(encoded: Vec<u8>) -> Result<String, &'static str> {
    let mut start = 0;
    let mut end = 8;
    let header_len = decode_len(&encoded[start..end]);
    start = end;
    end = header_len + 8;
    let table = decode_table(&encoded[start..end])?;
    start = end;
    end = start + 8;
    let text_len = decode_len(&encoded[start..end]);
    start = end;
    end = start + text_len;
    let text = decode_text(&encoded[start..end], &table)?;
    Ok(text)
}
fn decode_len(encoded: &[u8]) -> usize {
    let mut buf = [0u8; 8];
    for i in 0..8 {
        buf[i] = encoded[i];
    }
    usize::from_be_bytes(buf)
}

fn decode_table(encoded: &[u8]) -> Result<HashMap<String, char>, &'static str> {
    let mut entries = Vec::new();
    let mut last = 0;
    let mut pos = 0;
    for i in 0..encoded.len() {
        let byte = encoded[i];
        if byte == b',' && (last == b'0' || last == b'1') {
            entries.push(&encoded[pos..i]);
            pos = i + 1;
        }
        last = byte;
    }
    let mut table = HashMap::new();
    for entry in entries {
        let mut pos = None;
        for i in 0..entry.len() {
            let byte = entry[i];
            if byte == b':' {
                pos = Some(i);
            }
        }
        let pos = pos.ok_or("Table entry is missing colon")?;
        let (c, p) = entry.split_at(pos);
        let character = String::from_utf8_lossy(c)
            .chars()
            .next()
            .ok_or(
                "Table entry contai
            ns invalid char",
            )
            .map_err(|e| {
                println!("{entry:?}");
                e
            })?;
        let prefix = String::from_utf8_lossy(&p[1..]).to_string();
        table.insert(prefix, character);
    }
    Ok(table)
}

fn decode_text(encoded: &[u8], table: &HashMap<String, char>) -> Result<String, &'static str> {
    let bits = to_bit_vec(encoded);
    let mut min_len = usize::MAX;
    let mut max_len = 0;
    for (prefix, _) in table.iter() {
        if prefix.len() > max_len {
            max_len = prefix.len();
        }
        if prefix.len() < min_len {
            min_len = prefix.len();
        }
    }
    let mut text = String::with_capacity(encoded.len() * 2);
    let mut left = 0;
    while left + max_len < bits.len() {
        for right in (left + min_len)..=(left + max_len) {
            let s = &bits[left..right];
            let prefix = s
                .iter()
                .map(|b| match b {
                    true => '1',
                    false => '0',
                })
                .collect::<String>();
            if let Some(character) = table.get(&prefix) {
                text.push(*character);
                left = right;
            }
        }
    }
    Ok(text)
}

fn to_bit_vec(encoded: &[u8]) -> Vec<bool> {
    let mut bits = Vec::new();
    for byte in encoded {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            if bit == 1 {
                bits.push(true);
            } else {
                bits.push(false);
            }
        }
    }
    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_table() {
        let mut expected = HashMap::new();
        expected.insert('c', "1110".to_string());
        expected.insert('d', "101".to_string());
        expected.insert('e', "0".to_string());
        expected.insert('k', "111101".to_string());
        expected.insert('l', "110".to_string());
        expected.insert('m', "11111".to_string());
        expected.insert('u', "100".to_string());
        expected.insert('z', "111100".to_string());
        let encoded = encode_table(&expected);
        let decoded = decode_table(&encoded);
        assert!(decoded.is_ok());
        let decoded = decoded.unwrap();
        for (actual, prefix) in expected {
            let decoded_char = decoded.get(&prefix);
            println!("{prefix}");
            assert!(decoded_char.is_some());
            let actual = *decoded_char.unwrap();
            assert_eq!(actual, actual);
        }
    }
    #[test]
    fn encode_decode_text() {
        let text = "This is a text example which contains the special character , which is used to distinguish table entries in the header";
        let encoded = encode(text);
        let decoded = decode(encoded);
        assert!(decoded.is_ok());
        assert_eq!(text, decoded.unwrap());
    }
}
