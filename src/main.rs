use std::{process, time::{Duration, SystemTime}};

use anyhow::{Context, Result};
use crossterm::terminal;
use clap::Parser;
use matrix::waterfall::{Axis, MatrixWaterfall};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

mod cli;
mod matrix;
mod utils;

use cli::{command_line::{Args, TextColor, TextLanguage}, print::clear_screen};
use utils::color::Color;


fn main() -> Result<()> {
    let args = Args::parse();

    /*
     * Text Language
     */
    let mut language_set = "";

    if let Some(text_language) = args.language {
        language_set = text_language.to_char_set();
    }

    /*
     * Text Color
     */
    let (mut r, mut g, mut b): (u8, u8, u8) = (0, 0, 0);

    // specify a text color (e.g. -c red)
    if let Some(color) = args.color {
        (r, g, b) = color.to_rgb();
    }

    // if -r command entered
    if args.random {
        (r, g, b) = TextColor::random_rgb();
        language_set = TextLanguage::random_language();
        // println!("Random: ({}, {}, {})", r, g, b);
    }

    // if -R option entered
    let mut is_rainbow_mode = false;

    if args.rainbow {
        is_rainbow_mode = true;
    }

    let base_color = Color::from_rgb(r, g, b);

    /*
     * Text Size
     */
    let mut text_size_code = "\x1b[2m";

    if let Some(text_size) = args.size {
        text_size_code = text_size.to_ansi_escape_code();
    }

    /*
     * Text Speed
     */
    let mut velocity: u64 = 0;

    // specify a text speed (e.g. -s slow)
    if let Some(speed) = args.speed {
        velocity = speed.to_velocity();
    }

    /*
     * Threshold Density
     */
    let threshold_density = args.threshold_density;

    /*
     * Text Axis
     */
    let mut axis = Axis::Y;
    if let Some(text_axis) = args.axis {
        axis = text_axis.to_axis_enum();
    }

    // random number
    let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("time to have passed since UNIX_EPOCH").as_micros() as u64;
    let mut rand = Xoshiro256PlusPlus::seed_from_u64(seed);

    // waterfall
    let (width, height) = terminal::size().context("determine terminal size")?;

    let mut waterfall = match axis {
        Axis::Y => MatrixWaterfall::new_column(width, height, base_color, threshold_density, is_rainbow_mode),
        Axis::X => MatrixWaterfall::new_row(width, height, base_color, threshold_density, is_rainbow_mode)
    };

    let mut stdout = std::io::stdout();

    // shutdown the execution with Ctrl + C
    ctrlc::set_handler(move || {
        clear_screen();
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // start
    loop {
        waterfall.render_matrix(&mut stdout, text_size_code, &axis)?;
        waterfall.update_direction(language_set, &mut rand);
        std::thread::sleep(Duration::from_millis(velocity));
    }
}