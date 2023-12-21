use regex::Regex;

pub fn count_elves(input: &str) -> usize {
    let regex = Regex::new(r"elf").unwrap();
    regex.find_iter(input).count()
}

pub fn count_elves_on_a_shelf(input: &str) -> usize {
    let regex = Regex::new(r"elf on a ").unwrap();
    let mut count = 0;
    let mut start_search = 0;

    while let Some(mat) = regex.find_at(input, start_search) {
        let end = mat.end();
        if input[end..].starts_with("shelf") {
            count += 1;
        }
        start_search = end;
    }

    count
}

pub fn count_shelves_with_no_elf(input: &str) -> usize {
    let shelf_regex = Regex::new(r"shelf").unwrap();
    let elf_on_a_shelf = "elf on a ";

    shelf_regex
        .find_iter(input)
        .filter(|mat| {
            let start = mat.start();

            if start < elf_on_a_shelf.len() {
                return true;
            }

            &input[start - elf_on_a_shelf.len()..start] != elf_on_a_shelf
        })
        .count()
}
