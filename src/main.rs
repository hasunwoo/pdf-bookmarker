use std::{fs::OpenOptions, path::Path};

use std::io::Write;

use anyhow::Context;
use clap::Parser;
use command_parser::{Command, Subcommand};
use mupdf::{pdf::PdfDocument, Outline};
use pdf_utils::{
    apply_password, build_outline, build_toc, has_bookmark, has_password, remove_bookmarks,
    set_bookmark,
};
use toc_format::Toc;

mod command_parser;
mod pdf_utils;
mod toc_format;

fn main() {
    let cli = Command::parse();

    match &cli.command {
        Subcommand::Apply {
            input_file,
            toc_file,
            offset,
            password,
            force,
            write,
            output_file,
        } => {
            apply_subcommand(
                input_file,
                toc_file,
                *offset,
                password.as_deref(),
                *force,
                *write,
                output_file,
            );
        }
        Subcommand::Clear {
            input_file,
            password,
            write,
            output_file,
        } => {
            clear_subcommand(input_file, password.as_deref(), *write, output_file);
        }
        Subcommand::Extract {
            input_file,
            password,
            print,
            write,
            output_file,
        } => {
            extract_subcommand(
                input_file,
                password.as_deref(),
                *print,
                *write,
                output_file.as_deref(),
            );
        }
        Subcommand::Validate { toc_file } => {
            validate_subcommand(toc_file);
        }
    }
}

fn open_document(input_file: &str, password: Option<&str>) -> PdfDocument {
    let mut document = PdfDocument::open(input_file)
        .with_context(|| format!("Error opening input file: {input_file}"))
        .unwrap();

    match (has_password(&document).unwrap(), password) {
        (true, Some(password)) => match apply_password(&mut document, password) {
            Ok(true) => {}
            Ok(false) => {
                panic!("Password is invalid. Please provide a correct password.");
            }
            Err(err) => {
                panic!("An error occurred while processing the PDF: {}", err);
            }
        },
        (true, None) => {
            panic!("Document is password protected. please use -p or --password options to provide password.");
        }
        _ => {}
    }

    document
}

fn write_pdf(document: &PdfDocument, output_file: &str, force: bool) {
    if Path::new(output_file).exists() && !force {
        panic!("Output file is already exist. please use -w or --write options to replace output file.");
    }

    document
        .save(output_file)
        .with_context(|| format!("Error writing result PDF to {output_file}."))
        .unwrap();
}

fn write_toc(toc: &Toc, output_file: &str, force: bool) {
    if Path::new(output_file).exists() && !force {
        panic!("Output file is already exist. please use -w or --write options to replace output file.");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(force)
        .create_new(!force)
        .truncate(force)
        .open(output_file)
        .with_context(|| "Error opening file for writing: {output_file}")
        .unwrap();

    write!(file, "{toc}")
        .with_context(|| format!("Error writing result TOC file to {output_file}."))
        .unwrap();
}

fn build_outline_from_toc_file(toc_file: &str, page_offset: Option<i32>) -> Vec<Outline> {
    let toc: String = std::fs::read_to_string(toc_file)
        .with_context(|| format!("Error opening toc file: {toc_file}"))
        .unwrap();

    let mut toc: Toc = toc
        .parse()
        .with_context(|| "Error while parsing toc file")
        .unwrap();

    if let Some(page_offset) = page_offset {
        toc.page_offset(page_offset);
    }

    build_outline(&toc).unwrap()
}

fn apply_subcommand(
    input_file: &str,
    toc_file: &str,
    offset: Option<i32>,
    password: Option<&str>,
    force: bool, //overwrite outlines if exists in pdf.
    write: bool, //overwrite output file if output file already exists.
    output_file: &str,
) {
    let mut document = open_document(input_file, password);

    match (has_bookmark(&document).unwrap(), force) {
        (true, true) => {
            remove_bookmarks(&mut document)
                .with_context(|| "Error removing bookmark")
                .unwrap();
        }
        (true, false) => {
            panic!("Bookmark already exists. please use -f or --force options to overwrite existing bookmark.");
        }
        _ => {}
    }

    let outline = build_outline_from_toc_file(toc_file, offset);

    set_bookmark(&mut document, &outline)
        .with_context(|| "Error applying bookmarks to pdf file")
        .unwrap();

    write_pdf(&document, output_file, write);
}

fn clear_subcommand(input_file: &str, password: Option<&str>, write: bool, output_file: &str) {
    let mut document = open_document(input_file, password);

    remove_bookmarks(&mut document)
        .with_context(|| "Error removing bookmark")
        .unwrap();

    write_pdf(&document, output_file, write);
}

fn extract_subcommand(
    input_file: &str,
    password: Option<&str>,
    print: bool,
    write: bool,
    output_file: Option<&str>,
) {
    let document = open_document(input_file, password);

    let outline = document
        .outlines()
        .with_context(|| {
            "Error reading outlines from document. Maybe there is no outline or file is corrupted?"
        })
        .unwrap();

    let toc = build_toc(&outline)
        .with_context(|| "Error building TOC structure from outline")
        .unwrap();

    match (print, output_file) {
        (true, _) => {
            println!("{toc}");
        }
        (false, Some(output_file)) => {
            write_toc(&toc, output_file, write);
        }
        (false, None) => {
            panic!("Output file not specified. please use -o or --output to provide output file.")
        }
    }
}

fn validate_subcommand(toc_file: &str) {
    build_outline_from_toc_file(toc_file, None);
}
