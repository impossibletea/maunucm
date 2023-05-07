fn main() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_)  => input = r#"KpuH}I{"#.to_string()
    }

    let result: String = input
        .chars()
        .map(|a| maun_um(&a.to_string()))
        .collect();
    println!("{result}")
}

fn maun_um(letter: &str) -> String {
    String::from(match letter {
        "А"             => "A",
        "а"             => "a",
        "Б" | "б"       => "6",
        "В"             => "B",
        "в"             => "8",
        "Г"             => "I^",
        "г"             => "r",
        "Д"             => "D",
        "д"             => "g",
        "Е" | "Ё" | "Э" => "E",
        "е" | "ё" | "э" => "e",
        "Ж" | "ж"       => "}I{",
        "З" | "з"       => "3",
        "И" | "Й"       => "U",
        "и" | "й"       => "u",
        "К"             => "K",
        "к"             => "k",
        "Л" | "л"       => "JI",
        "М" | "м"       => "M",
        "Н" | "н"       => "H",
        "О"             => "O",
        "о"             => "o",
        "П"             => "II",
        "п"             => "n",
        "Р"             => "P",
        "р"             => "p",
        "С"             => "C",
        "с"             => "c",
        "Т"             => "T",
        "т"             => "m",
        "У"             => "Y",
        "у"             => "y",
        "Ф" | "ф"       => "%",
        "Х"             => "X",
        "х"             => "x",
        "Ц"             => "U,",
        "ц"             => "u,",
        "Ч" | "ч"       => "4",
        "Ш"             => "W",
        "ш"             => "w",
        "Щ"             => "W,",
        "щ"             => "w,",
        "Ъ" | "ъ"       => "'b",
        "Ы" | "ы"       => "bI",
        "Ь" | "ь"       => "b",
        "Ю"             => "IO",
        "ю"             => "io",
        "Я" | "я"       => "R",
        _ => letter
    })
}

