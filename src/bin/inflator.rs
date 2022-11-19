pub fn main() {
    let mut decoder = inflate::InflateWriter::from_zlib(std::io::stdout());
    std::io::copy(&mut std::io::stdin(), &mut decoder).unwrap();
}
