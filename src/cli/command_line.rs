use clap::{Parser, ValueEnum};
use rand::Rng;

use crate::matrix::waterfall::Axis;

#[derive(Parser, Debug)]
#[command(version, about, flatten_help = true)]
pub struct Args {
    /// specify a character
    #[arg(
        short = 'l',
        long = "language",
        value_enum,
        default_value = "default",
        conflicts_with = "random",
        value_parser = validate_language
    )]
    pub language: Option<TextLanguage>,

    /// specify a color
    #[arg(
        short = 'C',
        long = "color",
        value_enum,
        default_value = "green",
        conflicts_with = "random",
        conflicts_with = "rainbow",
        value_parser = validate_color
    )]
    pub color: Option<TextColor>,

    /// specify a size
    #[arg(
        short = 's',
        long = "size",
        value_enum,
        default_value = "small",
        value_parser = validate_size
    )]
    pub size: Option<TextSize>,

    /// specify a speed
    #[arg(
        short = 'v',
        long = "velocity",
        value_enum,
        default_value = "normal",
        value_parser = validate_speed
    )]
    pub speed: Option<TextSpeed>,

    /// specify a probability of threshold
    #[arg(
        short = 'd',
        long = "density",
        default_value_t = 0.05,
        value_parser = validate_density
    )]
    pub threshold_density: f32,

    /// specify a axis
    #[arg(
        short = 'a',
        long = "axis",
        value_enum,
        default_value = "y",
        value_parser = validate_axis
    )]
    pub axis: Option<TextAxis>,

    /// random color
    #[arg(
        short = 'r',
        long = "random",
        conflicts_with = "color",
        conflicts_with = "rainbow"
    )]
    pub random: bool,

    /// rainbow
    #[arg(
        short = 'R',
        long = "rainbow",
        conflicts_with = "color",
        conflicts_with = "random"
    )]
    pub rainbow: bool,
}

/// Validate a language option
fn validate_language(val: &str) -> Result<TextLanguage, String> {
    match val.to_lowercase().as_str() {
        "default" => Ok(TextLanguage::Default),
        "bin" => Ok(TextLanguage::Binary),
        "num" => Ok(TextLanguage::Numeric),
        "math" => Ok(TextLanguage::Math),
        "phy" => Ok(TextLanguage::Physics),
        "ja" => Ok(TextLanguage::Japanese),
        "en" => Ok(TextLanguage::English),
        "de" => Ok(TextLanguage::German),
        "ru" => Ok(TextLanguage::Russian),
        _ => Err(format!(
            "Invalid color: '{}'. Allowed values are: green, red, blue, yellow.",
            val
        ))
    }
}

/// Validate a color option
fn validate_color(val: &str) -> Result<TextColor, String> {
    match val.to_lowercase().as_str() {
        "green" => Ok(TextColor::Green),
        "red" => Ok(TextColor::Red),
        "blue" => Ok(TextColor::Blue),
        "yellow" => Ok(TextColor::Yellow),
        _ => Err(format!(
            "Invalid color: '{}'. Allowed values are: green, red, blue, yellow.",
            val
        ))
    }
}

/// Validate a size option
fn validate_size(val: &str) -> Result<TextSize, String> {
    match val.to_lowercase().as_str() {
        "small" => Ok(TextSize::Small),
        "medium" => Ok(TextSize::Medium),
        "large" => Ok(TextSize::Large),
        _ => Err(format!(
            "Invalid size: '{}'. Allowed values are: small, medium, large.",
            val
        ))
    }
}

/// Validate a speed option
fn validate_speed(val: &str) -> Result<TextSpeed, String> {
    match val.to_lowercase().as_str() {
        "normal" => Ok(TextSpeed::Normal),
        "slow" => Ok(TextSpeed::Slow),
        "fast" => Ok(TextSpeed::Fast),
        _ => Err(format!(
            "Invalid speed: '{}'. Allowed values are: normal, slow, fast.",
            val
        ))
    }
}

/// Validate a threshold density option
fn validate_density(val: &str) -> Result<f32, String> {
    let parsed: f32 = val.parse().map_err(|_| "Density must be a number".to_string())?;

    if (0.01..=1.0).contains(&parsed) {
        Ok(parsed)
    } else {
        Err("Density must be between 0.01 and 1.0".to_string())
    }
}

/// Validate an axis option
fn validate_axis(val: &str) -> Result<TextAxis, String> {
    match val.to_lowercase().as_str() {
        "y" => Ok(TextAxis::Y),
        "x" => Ok(TextAxis::X),
        _ => Err(format!(
            "Invalid axis: '{}'. Allowed values are: x or y.",
            val
        ))
    }
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum TextLanguage {
    Default,
    Binary,
    Numeric,
    Math,
    Physics,
    Japanese,
    English,
    German,
    Russian
}

impl TextLanguage {
    pub fn to_char_set(&self) -> &'static str {
        match self {
            TextLanguage::Default => "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍｦｲｸｺｿﾁﾄﾉﾌﾔﾖﾙﾚﾛﾝ012345789Z:.\"=*+-<>¦╌ç",
            TextLanguage::Binary => "01",
            TextLanguage::Numeric => "0123456789",
            TextLanguage::Math => "∑∫∂∞∇⊥πθαβγδ±√≠≈≡≪≫∈⊂⊃∩∪",
            TextLanguage::Physics => "ℏ≈μΩσℜℑτΔ∇⊗ℵρθλΣΔ∂⋅⊙",
            TextLanguage::Japanese => "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾊﾓﾏﾕﾚﾛﾝ",
            TextLanguage::English => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            TextLanguage::German => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzäöüßÄÖÜ",
            TextLanguage::Russian => "АБВГҐДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЫЭЮЯабвгдеёжзийклмнопqrstuvwxyz",
        }
    }

    pub fn random_language() -> &'static str {
        let languages = vec![
            TextLanguage::Binary,
            TextLanguage::Numeric,
            TextLanguage::Math,
            TextLanguage::Physics,
            TextLanguage::Japanese,
            TextLanguage::English,
            TextLanguage::German,
            TextLanguage::Russian,
        ];

        let mut rng = rand::thread_rng();
        languages[rng.gen_range(0..languages.len())].clone().to_char_set()
    }
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum TextColor {
    Green,
    Red,
    Blue,
    Yellow
}

impl TextColor {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            TextColor::Green => (0, 255, 43),    // Green
            TextColor::Red => (251, 46, 1),      // Red
            TextColor::Blue => (7, 71, 153),     // Blue
            TextColor::Yellow => (255, 191, 0),  // Yellow
        }
    }

    pub fn random_rgb() -> (u8, u8, u8) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255))
    }
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum TextSize {
    Small,
    Medium,
    Large
}

impl TextSize {
    pub fn to_ansi_escape_code(&self) -> &'static str {
        match self {
            TextSize::Small => "\x1b[2m",
            TextSize::Medium => "\x1b[22m",
            TextSize::Large => "\x1b[1m\x1b[3m"
        }
    }
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum TextSpeed {
    Normal,
    Slow,
    Fast
}

impl TextSpeed {
    pub fn to_velocity(&self) -> u64 {
        match self {
            TextSpeed::Normal => 75,
            TextSpeed::Slow => 120,
            TextSpeed::Fast => 45,
        }
    }
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum TextAxis {
    X,
    Y
}

impl TextAxis {
    pub fn to_axis_enum(&self) -> Axis {
        match self {
            TextAxis::Y => Axis::Y,
            TextAxis::X => Axis::X
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_help_message() {
        let mut cmd = Args::command();
        let help = cmd.render_long_help();
        let output = help.to_string();
        assert!(output.contains("--language"));
        assert!(output.contains("--color"));
        assert!(output.contains("--size"));
        assert!(output.contains("--velocity"));
        assert!(output.contains("--density"));
        assert!(output.contains("--random"));
        assert!(output.contains("--rainbow"));
    }

    #[test]
    fn test_default_options() {
        let args = Args::parse_from(&["program"]);

        assert_eq!(args.language, Some(TextLanguage::Default));
        assert_eq!(args.color, Some(TextColor::Green));
        assert_eq!(args.size, Some(TextSize::Small));
        assert_eq!(args.speed, Some(TextSpeed::Normal));
        assert_eq!(args.threshold_density, 0.05);
        assert_eq!(args.random, false);
        assert_eq!(args.rainbow, false);
    }

    #[test]
    fn test_short_language_option() {
        let args = Args::parse_from(&["program", "-l", "bin"]);
        assert_eq!(args.language, Some(TextLanguage::Binary));
    }

    #[test]
    fn test_long_language_option() {
        let args = Args::parse_from(&["program", "--language", "bin"]);
        assert_eq!(args.language, Some(TextLanguage::Binary));
    }

    #[test]
    fn test_language_out_of_option() {
        let result = Args::try_parse_from(&["program", "-language", "gre"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_short_color_option() {
        let args = Args::parse_from(&["program", "-C", "red"]);
        assert_eq!(args.color, Some(TextColor::Red));
    }

    #[test]
    fn test_long_color_option() {
        let args = Args::parse_from(&["program", "--color", "red"]);
        assert_eq!(args.color, Some(TextColor::Red));
    }

    #[test]
    fn test_color_out_of_option() {
        let result = Args::try_parse_from(&["program", "-C", "unknown_color"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_short_speed_option() {
        let args = Args::parse_from(&["program", "-v", "fast"]);
        assert_eq!(args.speed, Some(TextSpeed::Fast));
    }

    #[test]
    fn test_long_speed_option() {
        let args = Args::parse_from(&["program", "--velocity", "fast"]);
        assert_eq!(args.speed, Some(TextSpeed::Fast));
    }

    #[test]
    fn test_speed_out_of_option() {
        let result = Args::try_parse_from(&["program", "-s", "super_fast"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_short_random_option() {
        let args = Args::parse_from(&["program", "-r"]);
        assert!(args.random);
    }

    #[test]
    fn test_long_random_option() {
        let args = Args::parse_from(&["program", "--random"]);
        assert!(args.random);
    }

    #[test]
    fn test_short_rainbow_option() {
        let args = Args::parse_from(&["program", "-R"]);
        assert!(args.rainbow);
    }

    #[test]
    fn test_long_rainbow_option() {
        let args = Args::parse_from(&["program", "--rainbow"]);
        assert!(args.rainbow);
    }


    #[test]
    fn test_short_threshold_density_option() {
        let args = Args::parse_from(&["program", "-d", "0.2"]);
        assert_eq!(args.threshold_density, 0.2);
    }

    #[test]
    fn test_long_threshold_density_option() {
        let args = Args::parse_from(&["program", "--density", "0.2"]);
        assert_eq!(args.threshold_density, 0.2);
    }

    #[test]
    fn test_threshold_density_out_of_option() {
        let result = Args::try_parse_from(&["program", "-d", "10"]);
        assert!(result.is_err());
    }
}