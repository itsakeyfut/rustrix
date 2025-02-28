use std::io::Write;

use anyhow::{Context, Result};
use crossterm::{cursor, style, queue};
use rand::RngCore;

use crate::utils::color::Color;
use super::{column::Column, render::{Renderable, WaterfallElement}, row::Row};

#[derive()]
pub enum Axis {
    X,
    Y
}

#[allow(dead_code)]
pub struct MatrixWaterfall {
    pub width: u16,
    pub height: u16,
    pub base_color: Color,
    pub direction: Vec<WaterfallElement>,
}

impl MatrixWaterfall {
    pub fn new_column(width: u16, height: u16, base_color: Color, threshold_density: f32, rainbow_mode: bool) -> Self {
        Self {
            width,
            height,
            base_color,
            direction: (0..width)
                .map(|_| WaterfallElement::Column(Column::new(height, base_color, threshold_density, rainbow_mode)))
                .collect(),
        }
    }

    pub fn new_row(width: u16, height: u16, base_color: Color, threshold_density: f32, rainbow_mode: bool) -> Self {
        Self {
            width,
            height,
            base_color,
            direction: (0..width)
                .map(|_| WaterfallElement::Row(Row::new(width, base_color, threshold_density, rainbow_mode)))
                .collect(),
        }
    }

    pub fn render_matrix<W: Write>(&self, out: &mut W, text_size_code: &str, axis: &Axis) -> Result<()> {
        queue!(out, cursor::Hide)?;
        queue!(out, cursor::MoveTo(0, 0))?;
        queue!(
            out,
            style::SetBackgroundColor(style::Color::Rgb { r: 0, g: 0, b: 0})
        )?;

        match axis {
            Axis::Y => {
                // Render the columns for vertical movement (existing logic)
                for y in 0..self.height {
                    for column in &self.direction {
                        match column {
                            WaterfallElement::Column(column) => column.render_at_y(out, y, text_size_code)?,
                            _ => continue,  // Skip row rendering
                        }
                    }
                }
            }
            Axis::X => {
                // Render the rows for horizontal movement (left to right)
                for x in 0..self.height {
                    for row in &self.direction {
                        match row {
                            WaterfallElement::Row(row) => row.render_at_x(out, x, text_size_code)?,
                            _ => continue,  // Skip column rendering
                        }
                    }
                }
            }
        }

        queue!(out, style::ResetColor)?;
        queue!(out, cursor::Show)?;
        out.flush().context("flush output")?;
        Ok(())
    }

    pub fn update_direction<R: RngCore>(&mut self, language_set: &str, rand: &mut R) {
        for column in &mut self.direction {
            column.update_glyphs(language_set, rand)
        }
    }
}