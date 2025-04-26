use std::io::Write;

use anyhow::Result;
use rand::Rng;

use crate::utils::color::Color;
use super::glyph::Glyph;


#[derive(Clone)]
pub struct Row {
    height: u16,
    base_color: Color,
    glyphs: Vec<Glyph>,
    active_index: usize,
    threshold: f32,
    rainbow_mode: bool
}

impl Row {
    pub fn new(height: u16, base_color: Color, threshold: f32, rainbow_mode: bool) -> Self {
        Self {
            height,
            base_color,
            glyphs: vec![Glyph::clear(); height as usize],
            active_index: 0,
            threshold,
            rainbow_mode
        }
    }

    pub fn render_at_x<W: Write>(&self, out: &mut W, x: u16, text_size_code: &str) -> Result<()> {
        if (x as usize) >= self.height as usize {
            return Err(anyhow::anyhow!("x index out of bounds: {}", x).into());
        }

        self.glyphs[x as usize].display_glyphs(out, text_size_code)?;
        Ok(())
    }

    pub fn update_glyphs<R: Rng>(&mut self, language_set: &str, rand: &mut R) {
        // All glyphs faded
        for glyph in &mut self.glyphs {
            glyph.fade_color();
        }

        // Add conditions if not generate random numbers
        if self.active_index == 0 && rand.r#gen::<f32>() > self.threshold {
            return;
        }

        // Configured glyphs' color
        let color = if self.rainbow_mode {
            Color::from_rgb(rand.r#gen(), rand.r#gen(), rand.r#gen())
        } else {
            self.base_color
        };

        // Update glyphs if active_index is in the range of glyphs
        if self.active_index < self.height as usize {
            self.glyphs[self.active_index] = Glyph::generate_random_glyphs(rand, language_set, color);
        }

        // Increment active_index and reset if it exceeds height
        self.active_index += 1;

        // Reset active_index if it exceeds glyphs' length
        if self.active_index >= self.height as usize {
            self.active_index = 0;
        }
    }
}