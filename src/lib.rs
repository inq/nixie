extern crate clang;

use clang::{Clang, Index, EntityKind};

/// Find the main function and print the name and type.
pub fn process() -> Result<(), String> {
    let clang = Clang::new()?;
    let index = Index::new(&clang, false, false);
    let unit = index.parser("resources/test.cc").parse()?;
    let entity = unit.get_entity();

    for child in entity.get_children() {
        if child.is_in_main_file() {
            let tokens = child.get_range().unwrap().tokenize();
            let mut last_line = 1;
            let mut last_column = 1;
            for token in tokens {
                let range = token.get_range();
                let start = range.get_start().get_spelling_location();
                let end = range.get_end().get_spelling_location();
                let line = start.line;
                let width = end.column - start.column;
                if line != last_line {
                    last_line = line;
                    last_column = 1;
                    println!();
                }
                let spaces = String::from_utf8(vec![b' '; (start.column - last_column) as usize])
                    .unwrap();
                last_column = end.column;
                print!("{}{}", spaces, token.get_spelling());

            }
        }
    }
    Ok(())
}
