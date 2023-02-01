//! Tools to compile the book

pub mod index;
pub mod markdown;
pub mod options;
pub mod pandoc;

use std::path::Path;
use std::error::Error;
use helpers;
use convert_book::pandoc::save_as;

/// Render book in different formats
pub fn render_book(prefix: &str, src_path: &Path, meta_file: &str) -> Result<(), Box<Error>> {
    let meta_data = helpers::file::get_file_content(meta_file)?;

    let book = markdown::to_single_file(src_path,
                                        &meta_data.replace("{release_date}",
                                                           options::RELEASE_DATE))?;

    let _book_name = format!("dist/{}-{}.md", prefix, options::RELEASE_DATE);

    helpers::file::write_string_to_file(&book,
                                        &format!("dist/{}-{}.md",
                                                 prefix,
                                                 options::RELEASE_DATE))?;
    println!("[✓] {}", "MD");

    save_as(&book, prefix, "html", options::HTML)?;
    save_as(&book, prefix, "epub", options::EPUB)?;
    //save_as(&book_name, prefix, "html", options::HTML)?;
    //save_as(&book_name, prefix, "epub", options::EPUB)?;

    let cc_book = helpers::convert_checkmarks::convert_checkmarks(&book);
    save_as(&cc_book, prefix, "tex", options::LATEX)?;

    let plain_book = helpers::remove_emojis::remove_emojis(&cc_book);

     save_as(&plain_book,
            prefix,
            "letter.pdf",
            &format!(r"{} --variable papersize=letterpaper", options::LATEX))?;
    // save_as(&plain_book,
    //         prefix,
    //         "a4.pdf",
    //         &format!(r"{} --variable papersize=a4paper", options::LATEX))?;
    Ok(())
}
