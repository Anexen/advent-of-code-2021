fn main() {
    if let Some(selection) = std::env::args().nth(1) {
        match selection.as_str() {
            "1a" => day_01::part_a(),
            "1b" => day_01::part_b(),
            "2a" => day_02::part_a(),
            "2b" => day_02::part_b(),
            _ => unimplemented!(),
        }
    } else {
        println!("Usage: <day><part>, eg `cargo run -- 1a`");
    }
}
