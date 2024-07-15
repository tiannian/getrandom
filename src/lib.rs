#[derive(Debug)]
pub struct Error {}

pub fn getrandom(dest: &mut [u8]) -> Result<(), Error> {
    let bytes = include_bytes!("../random");

    dest.copy_from_slice(&bytes[..dest.len()]);

    println!("send random: {}", hex::encode(dest));

    Ok(())
}
