use anyhow::{anyhow, Context};
use mupdf::{pdf::PdfDocument, Outline};

use crate::toc_format::{Toc, TocEntry};

pub fn has_bookmark(document: &PdfDocument) -> anyhow::Result<bool> {
    document
        .outlines()
        .with_context(|| "Error reading outlines")
        .map(|outlines| !outlines.is_empty())
}

pub fn remove_bookmarks(document: &mut PdfDocument) -> anyhow::Result<()> {
    document
        .delete_outlines()
        .with_context(|| "Error removing existing outlines")
}

pub fn has_password(document: &PdfDocument) -> anyhow::Result<bool> {
    document
        .needs_password()
        .with_context(|| "Error while checking for password")
}

pub fn apply_password(document: &mut PdfDocument, password: &str) -> anyhow::Result<bool> {
    if !has_password(document)? {
        return Err(anyhow!("Document does not needs password"));
    }
    document
        .authenticate(password)
        .with_context(|| "Authentication error")
}

pub fn new_outline(title: String, page: u32) -> anyhow::Result<Outline> {
    if page == 0 {
        return Err(anyhow!("Invalid page number 0. page number starts from 1."));
    }
    Ok(Outline {
        title,
        uri: None,
        page: Some(page - 1),
        down: vec![],
        x: 0f32,
        y: 0f32,
    })
}

pub fn set_bookmark(document: &mut PdfDocument, outline: &[Outline]) -> anyhow::Result<()> {
    if !document
        .outlines()
        .with_context(|| "Error reading outlines")?
        .is_empty()
    {
        return Err(anyhow!(
            "Document already contains outlines. Please clear before apply new one."
        ));
    }
    document
        .set_outlines(outline)
        .with_context(|| "Error while setting new outlines")
}

// Toc -> Vec<Outline>
pub fn build_outline(toc: &Toc) -> anyhow::Result<Vec<Outline>> {
    if toc.entries.first().is_some_and(|e| e.depth != 0) {
        return Err(anyhow!("First entry in Toc format must be depth of 0."));
    }
    let mut index = 0;
    build_outline_recursive(&toc.entries, 0, &mut index)
}

fn build_outline_recursive(
    entries: &[TocEntry],
    depth: u32,
    index: &mut usize,
) -> anyhow::Result<Vec<Outline>> {
    let mut result = vec![];

    //traverse current depth. break when depth changes.
    while *index < entries.len() && entries[*index].depth == depth {
        let entry = &entries[*index];
        let mut outline = new_outline(entry.title.clone(), entry.page)?;

        //advance to next entry
        *index += 1;

        //recursively build outline for children if next node is deeper by one
        //check index for overflow
        if *index < entries.len() {
            let entry_depth = entries[*index].depth;
            //check if next entry gets deeper.
            if entry_depth > depth {
                //entry gets deeper by one. explore children.
                if entry_depth == depth + 1 {
                    outline.down = build_outline_recursive(entries, depth + 1, index)?;
                } else {
                    //entry is more deeper than 1. input data is not valid.
                    return Err(anyhow!(
                        "Invalid entry at {:?}. entry gets more deeper than 1.",
                        entries[*index]
                    ));
                }
            }
        }

        //push outline to result
        result.push(outline);
    }

    Ok(result)
}

// Vec<Outline> -> Toc
pub fn build_toc(outlines: &[Outline]) -> anyhow::Result<Toc> {
    let mut result = vec![];
    for outline in outlines {
        build_toc_recursive(outline, 0, &mut result)?;
    }
    Ok(Toc { entries: result })
}

fn build_toc_recursive(
    node: &Outline,
    depth: usize,
    result: &mut Vec<TocEntry>,
) -> anyhow::Result<()> {
    //visit outline
    result.push(TocEntry {
        depth: depth as u32,
        page: node.page.unwrap_or(1),
        title: node.title.clone(),
    });
    //visit child recursively
    for child in &node.down {
        build_toc_recursive(child, depth + 1, result)?;
    }
    Ok(())
}
