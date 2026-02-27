/// Validate and decode an Albanian NID from the command line.
///
/// Usage: cargo run --example check -- <NID>
fn main() {
    let nid = std::env::args().nth(1).expect("usage: check <NID>");
    match nidx::albania::decode(&nid) {
        Ok(info) => {
            println!("  birthday: {}", info.birthday);
            println!("       sex: {}", info.sex);
            println!("  national: {}", info.is_national);
        }
        Err(e) => eprintln!("invalid: {e}"),
    }
}
