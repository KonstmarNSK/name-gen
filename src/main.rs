use rand::Rng;

const CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g', 'h',
    'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r',
    's', 't', 'v', 'w', 'x', 'z',
];

const VOWELS: [char; 6] = [
    'a', 'e', 'i', 'o', 'u', 'y'
];

fn main() {
    let mut rng = rand::thread_rng();
    let length: u8 =  rng.gen_range(5..12);
    let mut result = String::with_capacity(length as usize);

    let mut prev_vowel = false;

    for _ in 0..length {
        if !&prev_vowel {
            result.push(VOWELS[rng.gen_range(0..6)].to_owned());
            prev_vowel = true;
        } else {
            result.push(CONSONANTS[rng.gen_range(0..20)].to_owned());
            prev_vowel = false
        }
    }

    println!("{}", result);
}
