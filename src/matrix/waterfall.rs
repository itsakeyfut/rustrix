use std::io::Write;

use anyhow::{Context, Result};
use crossterm::{cursor, style, queue};
use rand::RngCore;

use crate::utils::color::Color;
use super::column::Column;

#[allow(dead_code)]
pub struct MatrixWaterfall {
    width: u16,
    height: u16,
    base_color: Color,
    columns: Vec<Column>,
}

impl MatrixWaterfall {
    pub fn new(width: u16, height: u16, base_color: Color, threshold_density: f32, rainbow_mode: bool) -> Self {
        Self {
            width,
            height,
            base_color,
            // columns: vec![Column::new(height, base_color); width as usize]
            columns: (0..width)
                .map(|_| Column::new(height, base_color, threshold_density, rainbow_mode))
                .collect()
        }
    }

    pub fn render_matrix<W: Write>(&self, out: &mut W, text_size_code: &str) -> Result<()> {
        queue!(out, cursor::Hide)?;
        queue!(out, cursor::MoveTo(0, 0))?;
        queue!(
            out,
            style::SetBackgroundColor(style::Color::Rgb { r: 0, g: 0, b: 0})
        )?;

        for y in 0..self.height {
            for column in &self.columns {
                column.render_at_y(out, y, text_size_code)?;
            }
        }

        queue!(out, style::ResetColor)?;
        queue!(out, cursor::Show)?;
        out.flush().context("flush output")?;
        Ok(())
    }

    pub fn update_columns<R: RngCore>(&mut self, language_set: &str, rand: &mut R) {
        for column in &mut self.columns {
            column.update_glyphs(language_set, rand)
        }
    }
}