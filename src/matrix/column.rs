use std::io::Write;

use anyhow::Result;
use rand::Rng;

use crate::utils::color::Color;
use super::glyph::Glyph;


#[derive(Clone)]
pub struct Column {
    height: u16,
    base_color: Color,
    glyphs: Vec<Glyph>,
    active_index: usize,
    threshold: f32,
    rainbow_mode: bool
}

impl Column {
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

    pub fn render_at_y<W: Write>(&self, out: &mut W, y: u16, text_size_code: &str) -> Result<()> {
        self.glyphs[y as usize].display_glyphs(out, text_size_code)?;
        Ok(())
    }

    pub fn update_glyphs<R: Rng>(&mut self, language_set: &str, rand: &mut R) {
        for glyph in &mut self.glyphs {
            glyph.fade_color();
        }

        if self.active_index == 0 && rand.gen::<f32>() > self.threshold {
            return;
        }

        let color = if self.rainbow_mode {
            Color::from_rgb(rand.gen(), rand.gen(), rand.gen())
        } else {
            self.base_color
        };

        self.glyphs[self.active_index] = Glyph::generate_random_glyphs(rand, language_set, color);
        self.active_index += 1;

        if self.active_index >= self.height as usize {
            self.active_index = 0;
        }
    }
}