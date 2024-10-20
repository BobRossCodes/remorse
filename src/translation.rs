use std::collections::HashMap;

pub fn to_morse(s: &str) -> String {
    let morse_map = HashMap::from([
        ('A', ".-"),
        ('B', "-..."),
        ('C', "-.-."),
        ('D', "-.."),
        ('E', "."),
        ('F', "..-."),
        ('G', "--."),
        ('H', "...."),
        ('I', ".."),
        ('J', ".---"),
        ('K', "-.-"),
        ('L', ".-.."),
        ('M', "--"),
        ('N', "-."),
        ('O', "---"),
        ('P', ".--."),
        ('Q', "--.-"),
        ('R', ".-."),
        ('S', "..."),
        ('T', "-"),
        ('U', "..-"),
        ('V', "...-"),
        ('W', ".--"),
        ('X', "-..-"),
        ('Y', "-.--"),
        ('Z', "--.."),
        ('0', "-----"),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
        (' ', "/"),
    ]);

    let mut morse_parts = vec![];

    for c in s.chars() {
        if let Some(morse_part) = morse_map.get(&c.to_ascii_uppercase()) {
            morse_parts.push(morse_part.to_owned());
        } else {
            println!("Unsupported character '{}', skipping", c);
        }
    }

    morse_parts.join(" ")
}
