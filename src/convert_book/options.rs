pub const RELEASE_DATE: &'static str = "2019-05-04";

pub const MARKDOWN: &'static str = "markdown+smart+grid_tables+pipe_tables-simple_tables+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identifiers-inline_code_attributes";

pub const HTML:  &'static str = "--embed-resources --standalone --highlight-style=tango --table-of-contents --section-divs --template=lib/template.html --css=lib/pandoc.css --to=html5";

pub const EPUB:  &'static str = "--embed-resources --standalone --highlight-style=tango --css=lib/epub.css --table-of-contents";

pub const LATEX: &'static str = "--embed-resources --standalone --highlight-style=tango --top-level-division=chapter --table-of-contents --template=lib/template.tex --pdf-engine=xelatex --to=latex --pdf-engine-opt=--interaction=nonstopmode";
