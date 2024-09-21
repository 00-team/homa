pub fn toman(irr: i64) -> String {
    (irr / 10)
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .expect("utils::toman failed")
        .join(",")
}
