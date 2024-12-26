use std::io::Write;

use anyhow::{Context, Result};
use crossterm::{style, queue};
use rand::Rng;

use crate::utils::color::{Color, HslColor};

#[derive(Clone)]
pub struct Glyph {
    pub character: char,
    pub color: Color
}

impl Glyph {
    pub fn generate_random_glyphs<R: Rng>(rand: &mut R, language_set: &str, color: Color) -> Self {
        let chars: Vec<char> = language_set.chars().collect();
        let rng_idx = rand.gen_range(0..chars.len());

        Self {
            character: chars[rng_idx],
            color
        }
    }

    pub fn clear() -> Self {
        Self {
            character: ' ',
            color: Color::from_rgb(0, 0, 0)
        }
    }

    pub fn display_glyphs<W: Write>(&self, out: &mut W, text_size_code: &str) -> Result<()> {
        queue!(
            out,
            style::SetForegroundColor(style::Color::Rgb {
                r: self.color.r,
                g: self.color.g,
                b: self.color.b
            })
        )?;
        // write!(out, "\x1b[11m")?;
        write!(out, "{}", text_size_code)?;
        queue!(out, style::Print(self.character.to_string())).context("write glyph to output")?;
        Ok(())
    }

    pub fn fade_color(&mut self) {
        let hsl = self.color.as_hsl();
        let new_color = HslColor::new(hsl.h, hsl.s * 0.9, hsl.l * 0.9);
        if new_color.s < 9.0 || new_color.l < 9.0 {
            self.color = HslColor::new(hsl.h, 9.0, 9.0).into();
        } else {
            self.color = new_color.into();
        }
    }
}