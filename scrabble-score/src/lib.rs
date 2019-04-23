/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut result = 0;

    for i in word.to_lowercase().chars(){
        let score = match i {
            'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
            'd'|'g' => 2,
            'b'|'c'|'m'|'p' => 3,
            'f'|'h'|'v'|'w'|'y' => 4,
            'k' => 5,
            'j'|'x' => 8,
            'q'|'z' => 10,
            _ => 0,
        };
        result += score
    }
    result
}

//https://doc.rust-lang.org/1.5.0/book/match.html
//https://doc.rust-lang.org/rust-by-example/flow_control/match.html
