/*
    Homework from chapter 8

    Convert strings to pig latin. The first consonant of each word is moved to the end of the word
    and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the
    end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
 */
fn main() {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let phrase = "the quick brown fox jumps over the lazy dog";
    let words = phrase.split_whitespace();
    let mut pig_latin = String::new();

    for word in words {
        // Convert each word into characters using .chars() so we don't assume each letter is
        // 1 byte.
        let mut chars = word.chars();

        // Consume the first iteration of chars by calling chars.next()
        let (first_letter, letters) = match chars.next() {
            // If chars.next() is something add the first letter to a tuple then consume the rest of
            // the chars iterator using as_str() and return it as well in a tuple.
            // After this the `chars` iterator should be exhausted
            Some(c) => (c, chars.as_str()),
            // This should be unreachable!() as `split_whitespace()` should return valid words,
            // words should always have at least 1 character. So there is no instance where
            // `chars.next()` should be none in this case.
            //
            // Instead of calling unreachable!() or panic!() by calling continue if this unreachable
            // code somehow got reached it would just continue onto the next loop iteration without
            // crashing the program.
            None => continue,
        };

        if VOWELS.contains(&first_letter) {
            pig_latin.push_str(&format!("{first_letter}{letters}-hay "));
        } else {
            pig_latin.push_str(&format!("{letters}-{first_letter}ay "));
        }
    }

    // Remove whitespace added by the last iteration of word in words
    // There are other ways to solve this by just not adding whitespace on the last iteration
    // but trimming the result is just as easy
    let pig_latin_phrase = pig_latin.trim();

    println!("{phrase}");
    println!("{pig_latin_phrase}");
}