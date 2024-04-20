pub fn md5str(origin: &[u8]) -> String {
    let digest = md5::compute(origin);
    format!("{:x}", digest)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        println!("{}", md5str(b"hello"))
    }
}
