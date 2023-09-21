use clap::Parser;
use std::str;

#[derive(Parser)]
struct Cli {
    /// Messege to encrypt
    #[arg(short, long, value_name = "messege to encrypt")]
    messege: String,
    /// Key for encryption
    #[arg(short, long, value_name = "Key")]
    key: String,

    ///Encrypt or Decrypt
    #[arg(short, long, value_name = "Key", default_value_t = false)]
    d: bool,
}

fn gen_key(keyword: &str) -> Vec<u8> {
    let mut keyword: Vec<u8> = keyword.as_bytes().to_vec();
    keyword.sort();
    let mut new_key: Vec<u8> = vec![];
    new_key.push(keyword[0]);
    for i in 1..keyword.len() {
        let lst_ne = new_key.last();
        if keyword[i] != *lst_ne.unwrap() {
            new_key.push(keyword[i])
        } else {
            continue;
        }
    }
    new_key
}

fn create_gid(keyword: &str, messege: &str) -> Vec<Vec<u8>> {
    let key = gen_key(keyword);
    let num_columns = key.len();
    let num_rows = (messege.len() + &num_columns - 1) / num_columns;

    let mut grid: Vec<Vec<u8>> = vec![];
    for _ in 0..num_rows {
        grid.push(vec![0u8; num_columns]);
    }
    for (i, ch) in messege.as_bytes().iter().enumerate() {
        let row = i / num_columns;
        let col = i % num_columns;
        grid[row][col] = *ch;
    }
    grid
}

fn encrypt(keyword: &str, messege: &str) -> Vec<u8> {
    let key = gen_key(keyword);
    let grid = create_gid(keyword, messege);
    let mut ciphertext: Vec<u8> = vec![];
    for ch in &key {
        let col = key.iter().position(|&r| r == ch.clone()).unwrap();
        for row in 0..grid.len() {
            ciphertext.push(grid[row][col])
        }
    }
    ciphertext
}

fn decrypt(keyword: &str, ciphertext: Vec<u8>) -> Vec<u8> {
    let key = gen_key(keyword);
    let num_columns = key.len();
    let num_rows = (ciphertext.len() + num_columns - 1) / num_columns;

    let mut grid: Vec<Vec<u8>> = vec![];
    for _ in 0..num_rows {
        grid.push(vec![0u8; num_columns]);
    }

    let mut index = 0;
    for ch in &key {
        let col = key.iter().position(|&r| r == ch.clone()).unwrap();
        for row in 0..num_rows {
            if index < ciphertext.len() {
                grid[row][col] = ciphertext[index];
                index += 1;
            }
        }
    }
    let mut pltext = Vec::new();
    for row in grid {
        for j in row {
            pltext.push(j);
        }
    }
    pltext
}

fn main() {
    let args = Cli::parse();
    let key = &args.key;
    let messege = &args.messege;
    if args.d {
        let decrypted_message = decrypt(key, messege.as_bytes().to_vec());
        println!(
            "{:?}",
            str::from_utf8(&decrypted_message).unwrap().replace("\0", " ")
        );
    } else {
        let encrypted_message = encrypt(key, messege);
        println!("{:?}", str::from_utf8(&encrypted_message).unwrap().replace("\0", " "));
        let decrypted_message = decrypt(key, encrypted_message);
        println!(
            "{:?}",
            str::from_utf8(&decrypted_message).unwrap().replace("\0", " ")
        );
    }
}
