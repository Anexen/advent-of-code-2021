fn main() {
    if let Some(selection) = std::env::args().nth(1) {
        let result = match selection.as_str() {
            "1a" => day_01::part_a(),
            "1b" => day_01::part_b(),
            "2a" => day_02::part_a(),
            "2b" => day_02::part_b(),
            // "3a" => day_03::part_a(),
            // "3b" => day_03::part_b(),
            // "4a" => day_04::part_a(),
            // "4b" => day_04::part_b(),
            // "5a" => day_05::part_a(),
            // "5b" => day_05::part_b(),
            // "6a" => day_06::part_a(),
            // "6b" => day_06::part_b(),
            "7a" => day_07::part_a(None),
            "7b" => day_07::part_b(None),
            _ => unimplemented!(),
        };
        println!("{}", result);
    } else {
        println!("Usage: <day><part>, eg `cargo run -- 1a`");
    }
}
