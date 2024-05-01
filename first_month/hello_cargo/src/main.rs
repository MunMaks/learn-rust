
fn main() {
    let s: String = String::from("Gustave Eiffel"); 
    let longueur: usize = first_word(&s);
    println!("{s}, len: {}", longueur);

}

/**
 * return la longueur du premier mot rencontre
*/
fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
