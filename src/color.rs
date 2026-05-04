// Color Definitions

use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

// `#[repr(transparent)]` ensures that AnsiColor has the exact same memory layout as an u8, making it zero-cost to wrap

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnsiColor(u8);

impl AnsiColor {
    /// Construct new `AnsiColor`
    ///
    /// # Example
    ///
    /// ```
    /// use asciigraph::AnsiColor;
    ///
    /// let color = AnsiColor::new(13);
    ///
    /// assert_eq!(color.code(), 13);
    /// ```
    pub fn new(number: u8) -> Self {
        AnsiColor(number)
    }

    pub fn code(&self) -> u8 {
        self.0
    }

    // Mapping the Ansi Color name to color names provided in string.
    // Output will be Some(AnsiColor(0)) for "default".
    // To get the number, it's AnsiColor::Default.code()
    //
    /// # Example
    ///
    /// ```
    /// use asciigraph::AnsiColor;
    ///
    /// let color = AnsiColor::AQUA;
    /// let color_from_str = AnsiColor::get_ansi_color("aqua");
    ///
    /// assert!(color_from_str.is_some());
    /// assert_eq!(color, color_from_str.unwrap());
    /// ```
    pub fn get_ansi_color(color: &str) -> Option<AnsiColor> {
        // .ok() converts Result -> Option
        // will work the same as before.
        //
        // in library crate from_str usually return `Result` not `Option`
        //
        // implementing [`FromStr`] and [`TryFrom`] is the idiomatic way of
        // building a Type from &str.
        AnsiColor::from_str(color).ok()
    }

    // Named ANSI 256-color constants
}

// `$($name:ident => $code:expr),*`
// - take many `COLOR => 01`
//
// ` $(,)?`
// - for the very last item the user may or may not put a comma.
//   Don't take it seriously.
macro_rules! __impl_const {
    ( $($name:ident => $code:expr),* $(,)? ) => {
        impl AnsiColor {
            $(
                pub const $name: AnsiColor = AnsiColor($code);
            )*
        }
    }
}

macro_rules! __impl_from_str {
    ( $($name:ident => $code:expr),* $(,)? ) => {
        /// # Example
        ///
        /// ```
        /// use asciigraph::AnsiColor;
        /// use std::str::FromStr;
        ///
        /// let color = AnsiColor::AQUA;
        /// let color_from_str = AnsiColor::from_str("aqua");
        /// let color_parsed = "aqua".parse::<AnsiColor>();
        ///
        /// assert!(color_from_str.is_ok());
        /// assert_eq!(color, color_from_str.unwrap());
        /// assert_eq!(color, color_parsed.unwrap());
        /// ```
        ///
        /// # Error
        ///
        /// return `Err()` on invalid color name.
        /// ```
        /// use asciigraph::AnsiColor;
        /// use std::str::FromStr;
        ///
        /// let not_a_color = AnsiColor::from_str("ironman");
        /// assert_eq!(not_a_color, Err("invalid color name"));
        /// ```
        impl FromStr for AnsiColor {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // sanitizing `&str` will make api more user-friendly
                // now "AliceBlue", "aliceblue", or "ALICEBLUE" will just work.
                match s {
                    "default" => return Ok(AnsiColor::default()),
                    $(
                        stringify!($name) => return Ok(AnsiColor::$name),
                    )*
                    _ => {
                        $(
                            if color_match(s, stringify!($name)) {

                                return Ok(AnsiColor::$name);
                            }
                        )*
                    }
                }
                Err("invalid color name")
            }
        }
    }
}

// impl CONST and generate FromStr dynamically at compile time.
macro_rules! define_colors {
    ( $($name:ident => $code:expr),* $(,)? ) => {
        __impl_const!( $($name => $code),* );
        __impl_from_str!( $($name => $code),* );
    }
}

#[rustfmt::skip] // don't format this block
define_colors! [
    AQUA => 14,
    ALICE_BLUE => 255,
    ANTIQUE_WHITE => 255,
    AQUAMARINE => 122,
    AZURE => 15,
    BEIGE => 230,
    BISQUE => 224,
    BLACK => 188,
    BLANCHED_ALMOND => 230,
    BLUE => 12,
    BLUE_VIOLET => 92,
    BROWN => 88,
    BURLY_WOOD => 180,
    CADET_BLUE => 73,
    CHARTREUSE => 118,
    CHOCOLATE => 166,
    CORAL => 209,
    CORNFLOWER_BLUE => 68,
    CORNSILK => 230,
    CRIMSON => 161,
    CYAN => 14,
    DARK_BLUE => 18,
    DARK_CYAN => 30,
    DARK_GOLDENROD => 136,
    DARK_GRAY => 248,
    DARK_GREEN => 22,
    DARK_KHAKI => 143,
    DARK_MAGENTA => 90,
    DARK_OLIVE_GREEN => 59,
    DARK_ORANGE => 208,
    DARK_ORCHID => 134,
    DARK_RED => 88,
    DARK_SALMON => 173,
    DARK_SEA_GREEN => 108,
    DARK_SLATE_BLUE => 60,
    DARK_SLATE_GRAY => 238,
    DARK_TURQUOISE => 44,
    DARK_VIOLET => 92,
    DEEP_PINK => 198,
    DEEP_SKY_BLUE => 39,
    DEFAULT => 0,
    DIM_GRAY => 242,
    DODGER_BLUE => 33,
    FIREBRICK => 124,
    FLORAL_WHITE => 15,
    FOREST_GREEN => 28,
    FUCHSIA => 13,
    GAINSBORO => 253,
    GHOST_WHITE => 15,
    GOLD => 220,
    GOLDENROD => 178,
    GRAY=> 8,
    GREEN=> 2,
    GREEN_YELLOW => 155,
    HONEYDEW => 15,
    HOT_PINK => 205,
    INDIAN_RED => 167,
    INDIGO => 54,
    IVORY => 15,
    KHAKI => 222,
    LAVENDER => 254,
    LAVENDER_BLUSH => 255,
    LAWN_GREEN => 118,
    LEMON_CHIFFON => 230,
    LIGHT_BLUE => 152,
    LIGHT_CORAL => 210,
    LIGHT_CYAN => 195,
    LIGHT_GOLDENROD_YELLOW => 230,
    LIGHT_GRAY  => 252,
    LIGHT_GREEN => 120,
    LIGHT_PINK => 217,
    LIGHT_SALMON => 216,
    LIGHT_SEA_GREEN => 37,
    LIGHT_SKY_BLUE => 117,
    LIGHT_SLATE_GRAY => 103,
    LIGHT_STEEL_BLUE => 152,
    LIGHT_YELLOW => 230,
    LIME => 10,
    LIME_GREEN => 77,
    LINEN => 255,
    MAGENTA => 13,
    MAROON => 1,
    MEDIUM_AQUAMARINE => 79,
    MEDIUM_BLUE => 20,
    MEDIUM_ORCHID => 134,
    MEDIUM_PURPLE => 98,
    MEDIUM_SEA_GREEN => 72,
    MEDIUM_SLATE_BLUE => 99,
    MEDIUM_SPRING_GREEN => 48,
    MEDIUM_TURQUOISE => 80,
    MEDIUM_VIOLET_RED => 162,
    MIDNIGHT_BLUE => 17,
    MINT_CREAM => 15,
    MISTY_ROSE => 224,
    MOCCASIN => 223,
    NAVAJO_WHITE => 223,
    NAVY => 4,
    OLD_LACE => 230,
    OLIVE => 3,
    OLIVE_DRAB => 64,
    ORANGE => 214,
    ORANGE_RED => 202,
    ORCHID => 170,
    PALE_GOLDENROD => 223,
    PALE_GREEN => 120,
    PALE_TURQUOISE => 159,
    PALE_VIOLET_RED => 168,
    PAPAYA_WHIP => 230,
    PEACH_PUFF => 223,
    PERU => 173,
    PINK => 218,
    PLUM => 182,
    POWDER_BLUE => 152,
    PURPLE => 5,
    RED => 9,
    ROSY_BROWN => 138,
    ROYAL_BLUE => 63,
    SADDLE_BROWN => 94,
    SALMON => 210,
    SANDY_BROWN => 215,
    SEA_GREEN => 29,
    SEA_SHELL => 15,
    SIENNA => 131,
    SILVER => 7,
    SKY_BLUE => 117,
    SLATE_BLUE => 62,
    SLATE_GRAY => 66,
    SNOW => 15,
    SPRING_GREEN => 48,
    STEEL_BLUE => 67,
    TAN => 180,
    TEAL => 6,
    THISTLE => 182,
    TOMATO => 203,
    TURQUOISE => 80,
    VIOLET => 213,
    WHEAT => 223,
    WHITE => 15,
    WHITE_SMOKE => 255,
    YELLOW => 11,
    YELLOW_GREEN => 149,
];
    
// deligates to [`FromStr`]
//
/// # Example
///
/// ```
/// use asciigraph::AnsiColor;
///
/// let color = AnsiColor::AQUA;
/// let color_from_str = AnsiColor::try_from("aqua");
///
/// assert!(color_from_str.is_ok());
/// assert_eq!(color, color_from_str.unwrap());
/// ```
impl TryFrom<&str> for AnsiColor {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AnsiColor::from_str(value)
    }
}

// Allow easy conversion from u8 to AnsiColor
impl From<u8> for AnsiColor {
    fn from(value: u8) -> Self {
        AnsiColor(value)
    }
}

// Allow easy conversion from AnsiColor to u8
impl From<AnsiColor> for u8 {
    fn from(color: AnsiColor) -> Self {
        color.0
    }
}

impl fmt::Display for AnsiColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if *self == Self::DEFAULT {
            return write!(f, "\x1b[0m");
        }

        let c = if *self == AnsiColor::BLACK {
            0u8
        } else {
            self.0
        };

        if c <= AnsiColor::SILVER.into() {
            write!(f, "\x1b[{}m", 30 + c)
        } else if c <= AnsiColor::WHITE.into() {
            write!(f, "\x1b[{}m", 82 + c)
        } else {
            write!(f, "\x1b[38;5;{}m", c)
        }
    }
}

/// compile time `str` check for `define_colors!` macro
const fn color_match(input: &str, target: &str) -> bool {
    debug_assert!(input.is_ascii());
    debug_assert!(target.is_ascii());

    let inp = input.as_bytes();
    let tar = target.as_bytes();

    let mut i = 0;
    let mut j = 0;

    while i < inp.len() || j < tar.len() {
        // skip '_' in input
        if i < inp.len() && inp[i] == b'_' {
            i += 1;
            continue;
        }
        // skip '_' in target
        if j < tar.len() && tar[j] == b'_' {
            j += 1;
            continue;
        }

        // If str len of input and target is different
        // after removing '_', return false

        if i >= inp.len() || j >= tar.len() {
            return false;
        }

        let b_inp = inp[i] | 0x20;
        let b_tar = tar[j] | 0x20;

        if b_inp != b_tar {
            return false;
        }

        i += 1;
        j += 1;
    }

    true
}
