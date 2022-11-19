use sha1::{Digest, Sha1};
fn main() {
    let input = "hello\n";
    println!("raw: {}", sha1digest(input.as_bytes()));

    let blob = format!("blob {}\0{}", input.len(), input);
    println!("blob: {}", sha1digest(blob.as_bytes()));

    let zipped = deflate::deflate_bytes_zlib(blob.as_bytes());
    println!("zipped: {}", sha1digest(&zipped));
}

fn sha1digest(input: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}
