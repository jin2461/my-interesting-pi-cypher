use std::fs::File;
use std::io;
use std::io::prelude::*;

fn string_to_digits(s: &str) -> Vec<u8> {
    s.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect()
}
//credit to chatgpt for creating these functions
fn letter_to_num(c: char) -> i32 {
    if c == ' ' || c == '$' {
        0
    } else {
        c as i32 - 'a' as i32 + 1
    }
}

fn num_to_letter(n: i32) -> char {
    if n == 0 {
        '$'
    } else {
        (n as u8 + 'a' as u8 - 1) as char
    }
}

fn letters_to_numbers(letters: Vec<char>) -> Vec<i32> {
    letters.into_iter().map(letter_to_num).collect()
}

fn numbers_to_letters(numbers: Vec<i32>) -> Vec<char> {
    numbers.into_iter().map(num_to_letter).collect()
}

fn encrypt(pi2: Vec<u8>, original_messege: String, num_key: usize) {
    let mut count: usize = 0;
    let mut i: usize = 0;
    let mut vector_of_letters_from_messege: Vec<char> = original_messege.chars().collect();
    let mut vector_of_numbers_from_messege: Vec<i32> =
        letters_to_numbers(vector_of_letters_from_messege);
    while i < (original_messege.len()) {
        let pi_offset: u8 = (pi2[i + count + num_key] * 10) + pi2[i + count + 1 + num_key];
        let pi_offset_as_i32: i32 = pi_offset as i32;
        vector_of_numbers_from_messege[i] =
            (vector_of_numbers_from_messege[i] + pi_offset_as_i32) % 27;
        count += 1;
        i += 1;
    }
    vector_of_letters_from_messege = numbers_to_letters(vector_of_numbers_from_messege);
    let incripted_messege: String = vector_of_letters_from_messege.into_iter().collect();
    println!("{incripted_messege}")
}

fn decrypt(pi2: Vec<u8>, original_messege: String, num_key: usize) {
    let mut count: usize = 0;
    let mut i: usize = 0;
    let mut vector_of_letters_from_messege: Vec<char> = original_messege.chars().collect();
    let mut vector_of_numbers_from_messege: Vec<i32> =
        letters_to_numbers(vector_of_letters_from_messege);
    while i < (original_messege.len()) {
        let pi_offset: u8 = (pi2[i + count + num_key] * 10) + pi2[i + count + 1 + num_key];
        let pi_offset_as_i32: i32 = pi_offset as i32;
        let mut vector_of_numbers_from_messege_minus_pi_offset: i32 =
            vector_of_numbers_from_messege[i] - pi_offset_as_i32;
        if vector_of_numbers_from_messege_minus_pi_offset > 0 {
            vector_of_numbers_from_messege_minus_pi_offset =
                vector_of_numbers_from_messege_minus_pi_offset % 27
        } else if (vector_of_numbers_from_messege_minus_pi_offset % 27) == 0 {
            vector_of_numbers_from_messege_minus_pi_offset = 0
        } else {
            vector_of_numbers_from_messege_minus_pi_offset =
                27 - ((vector_of_numbers_from_messege_minus_pi_offset.abs()) % 27);
        }
        vector_of_numbers_from_messege[i] = vector_of_numbers_from_messege_minus_pi_offset;
        count += 1;
        i += 1;
    }
    vector_of_letters_from_messege = numbers_to_letters(vector_of_numbers_from_messege);
    let decripted_messege: String = vector_of_letters_from_messege.into_iter().collect();
    let decripted_messege_with_space = decripted_messege.replace("$", " ");
    println!("{}", decripted_messege_with_space);
}

fn main() {
    println!("press 1 for incryption, press 2 for decrytion");
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("failed to readline");
    let mut f = File::open("pi1b.txt").expect("file not found");
    let mut pi: String = String::new();
    f.read_to_string(&mut pi).expect("nothing in file");
    println!("input your messege");
    let mut original_messege: String = String::new();
    io::stdin()
        .read_line(&mut original_messege)
        .expect("failed to readline");
    original_messege.pop();
    original_messege.pop();
    println!("what is the key?");
    let mut string_key: String = String::new();
    io::stdin()
        .read_line(&mut string_key)
        .expect("failed to readline");
    string_key.pop();
    string_key.pop();
    let num_key: usize = string_key.parse().expect("this is not an intiger");
    let pi2: Vec<u8> = string_to_digits(&pi);

    choice.pop();
    choice.pop();
    if choice == "1" {
        encrypt(pi2, original_messege, num_key);
    } else {
        decrypt(pi2, original_messege, num_key);
    }
}
