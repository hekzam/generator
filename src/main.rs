use sha3::{Shake128, digest::{Update, ExtendableOutput, XofReader}};
use qrcode_generator::{QrCodeEcc};

const HASH_SIZE: usize = 16;

type Hash = [u8; HASH_SIZE];

fn gen_hash(data: &[u8]) -> Hash {
    let mut hasher = Shake128::default(); // Up to 256 bits
    hasher.update(data);
    let mut reader = hasher.finalize_xof();
    let mut res1 = Hash::default(); // Since SHAKE is of variable d, we have to specify the output size.
    reader.read(&mut res1);
    return res1;
}

fn gen_qr(contents: Vec<&str>, page: u8) {
    // Generate hash
    let hash = gen_hash(
        &contents
            .into_iter()
            .flat_map(|v| v.as_bytes().to_owned()) // One may want to do this differently, here, [file1f, ile2] and [file1, file2] would output the same result.
            .collect::<Vec<u8>>()
    );

    // Create the full data (here, hash + page number)
    let mut data = hash.to_vec();
    data.push(page);

    // Write qr code
    qrcode_generator::to_png_to_file(data, QrCodeEcc::Medium, 1024, "tests/data/file_output.png").unwrap();

}

fn main() {
    gen_qr(vec!["test that is very super safe"], 1);
}