use std::io::Write;

use anyhow::Result;
use rand::Rng;

use super::{column::Column, row::Row};

pub enum WaterfallElement {
    Column(Column),
    Row(Row),
}

#[allow(dead_code)]
pub trait Renderable {
    fn render_at<W: Write>(&self, out: &mut W, pos: u16, text_size_code: &str) -> Result<()>;
    fn update_glyphs<R: Rng>(&mut self, language_set: &str, rand: &mut R);
}

impl Renderable for WaterfallElement {
    fn render_at<W: Write>(&self, out: &mut W, pos: u16, text_size_code: &str) -> Result<()> {
        match self {
            WaterfallElement::Column(column) => column.render_at_y(out, pos, text_size_code),
            WaterfallElement::Row(row) => row.render_at_x(out, pos, text_size_code),
        }
    }

    fn update_glyphs<R: Rng>(&mut self, language_set: &str, rand: &mut R) {
        match self {
            WaterfallElement::Column(column) => column.update_glyphs(language_set, rand),
            WaterfallElement::Row(row) => row.update_glyphs(language_set, rand),
        }
    }
}