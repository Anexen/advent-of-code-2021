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
            "8a" => day_08::part_a(None),
            "8b" => day_08::part_b(None),
            "9a" => day_09::part_a(None),
            "9b" => day_09::part_b(None),
            "10a" => day_10::part_a(None),
            "10b" => day_10::part_b(None),
            "11a" => day_11::part_a(None),
            "11b" => day_11::part_b(None),
            "13a" => day_13::part_a(None),
            "13b" => day_13::part_b(None),
            "14a" => day_14::part_a(None),
            "14b" => day_14::part_b(None),
            _ => unimplemented!(),
        };
        println!("{}", result);
    } else {
        println!("Usage: <day><part>, eg `cargo run -- 1a`");
    }
}
