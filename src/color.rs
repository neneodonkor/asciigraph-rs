// Color Definitions

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnsiColor(u8);

impl AnsiColor {
    pub fn new(number: u8) -> Self {
        AnsiColor(number)
    }

    pub fn code(&self) -> u8 {
        self.0
    }

    // Named ANSI 256-color constants
    pub const DEFAULT: AnsiColor = AnsiColor(0);
    pub const ALICE_BLUE: AnsiColor = AnsiColor(255);
    pub const ANTIQUE_WHITE: AnsiColor = AnsiColor(255);
    pub const AQUA: AnsiColor = AnsiColor(14);
    pub const AQUAMARINE: AnsiColor = AnsiColor(122);
    pub const AZURE: AnsiColor = AnsiColor(15);
    pub const BEIGE: AnsiColor = AnsiColor(230);
    pub const BISQUE: AnsiColor = AnsiColor(224);
    pub const BLACK: AnsiColor = AnsiColor(188); // dummy value
    pub const BLANCHED_ALMOND: AnsiColor = AnsiColor(230);
    pub const BLUE: AnsiColor = AnsiColor(12);
    pub const BLUE_VIOLET: AnsiColor = AnsiColor(92);
    pub const BROWN: AnsiColor = AnsiColor(88);
    pub const BURLY_WOOD: AnsiColor = AnsiColor(180);
    pub const CADET_BLUE: AnsiColor = AnsiColor(73);
    pub const CHARTREUSE: AnsiColor = AnsiColor(118);
    pub const CHOCOLATE: AnsiColor = AnsiColor(166);
    pub const CORAL: AnsiColor = AnsiColor(209);
    pub const CORNFLOWER_BLUE: AnsiColor = AnsiColor(68);
    pub const CORNSILK: AnsiColor = AnsiColor(230);
    pub const CRIMSON: AnsiColor = AnsiColor(161);
    pub const CYAN: AnsiColor = AnsiColor(14);
    pub const DARK_BLUE: AnsiColor = AnsiColor(18);
    pub const DARK_CYAN: AnsiColor = AnsiColor(30);
    pub const DARK_GOLDENROD: AnsiColor = AnsiColor(136);
    pub const DARK_GRAY: AnsiColor = AnsiColor(248);
    pub const DARK_GREEN: AnsiColor = AnsiColor(22);
    pub const DARK_KHAKI: AnsiColor = AnsiColor(143);
    pub const DARK_MAGENTA: AnsiColor = AnsiColor(90);
    pub const DARK_OLIVE_GREEN: AnsiColor = AnsiColor(59);
    pub const DARK_ORANGE: AnsiColor = AnsiColor(208);
    pub const DARK_ORCHID: AnsiColor = AnsiColor(134);
    pub const DARK_RED: AnsiColor = AnsiColor(88);
    pub const DARK_SALMON: AnsiColor = AnsiColor(173);
    pub const DARK_SEA_GREEN: AnsiColor = AnsiColor(108);
    pub const DARK_SLATE_BLUE: AnsiColor = AnsiColor(60);
    pub const DARK_SLATE_GRAY: AnsiColor = AnsiColor(238);
    pub const DARK_TURQUOISE: AnsiColor = AnsiColor(44);
    pub const DARK_VIOLET: AnsiColor = AnsiColor(92);
    pub const DEEP_PINK: AnsiColor = AnsiColor(198);
    pub const DEEP_SKY_BLUE: AnsiColor = AnsiColor(39);
    pub const DIM_GRAY: AnsiColor = AnsiColor(242);
    pub const DODGER_BLUE: AnsiColor = AnsiColor(33);
    pub const FIREBRICK: AnsiColor = AnsiColor(124);
    pub const FLORAL_WHITE: AnsiColor = AnsiColor(15);
    pub const FOREST_GREEN: AnsiColor = AnsiColor(28);
    pub const FUCHSIA: AnsiColor = AnsiColor(13);
    pub const GAINSBORO: AnsiColor = AnsiColor(253);
    pub const GHOST_WHITE: AnsiColor = AnsiColor(15);
    pub const GOLD: AnsiColor = AnsiColor(220);
    pub const GOLDENROD: AnsiColor = AnsiColor(178);
    pub const GRAY: AnsiColor = AnsiColor(8);
    pub const GREEN: AnsiColor = AnsiColor(2);
    pub const GREEN_YELLOW: AnsiColor = AnsiColor(155);
    pub const HONEYDEW: AnsiColor = AnsiColor(15);
    pub const HOT_PINK: AnsiColor = AnsiColor(205);
    pub const INDIAN_RED: AnsiColor = AnsiColor(167);
    pub const INDIGO: AnsiColor = AnsiColor(54);
    pub const IVORY: AnsiColor = AnsiColor(15);
    pub const KHAKI: AnsiColor = AnsiColor(222);
    pub const LAVENDER: AnsiColor = AnsiColor(254);
    pub const LAVENDER_BLUSH: AnsiColor = AnsiColor(255);
    pub const LAWN_GREEN: AnsiColor = AnsiColor(118);
    pub const LEMON_CHIFFON: AnsiColor = AnsiColor(230);
    pub const LIGHT_BLUE: AnsiColor = AnsiColor(152);
    pub const LIGHT_CORAL: AnsiColor = AnsiColor(210);
    pub const LIGHT_CYAN: AnsiColor = AnsiColor(195);
    pub const LIGHT_GOLDENROD_YELLOW: AnsiColor = AnsiColor(230);
    pub const LIGHT_GRAY: AnsiColor = AnsiColor(252);
    pub const LIGHT_GREEN: AnsiColor = AnsiColor(120);
    pub const LIGHT_PINK: AnsiColor = AnsiColor(217);
    pub const LIGHT_SALMON: AnsiColor = AnsiColor(216);
    pub const LIGHT_SEA_GREEN: AnsiColor = AnsiColor(37);
    pub const LIGHT_SKY_BLUE: AnsiColor = AnsiColor(117);
    pub const LIGHT_SLATE_GRAY: AnsiColor = AnsiColor(103);
    pub const LIGHT_STEEL_BLUE: AnsiColor = AnsiColor(152);
    pub const LIGHT_YELLOW: AnsiColor = AnsiColor(230);
    pub const LIME: AnsiColor = AnsiColor(10);
    pub const LIME_GREEN: AnsiColor = AnsiColor(77);
    pub const LINEN: AnsiColor = AnsiColor(255);
    pub const MAGENTA: AnsiColor = AnsiColor(13);
    pub const MAROON: AnsiColor = AnsiColor(1);
    pub const MEDIUM_AQUAMARINE: AnsiColor = AnsiColor(79);
    pub const MEDIUM_BLUE: AnsiColor = AnsiColor(20);
    pub const MEDIUM_ORCHID: AnsiColor = AnsiColor(134);
    pub const MEDIUM_PURPLE: AnsiColor = AnsiColor(98);
    pub const MEDIUM_SEA_GREEN: AnsiColor = AnsiColor(72);
    pub const MEDIUM_SLATE_BLUE: AnsiColor = AnsiColor(99);
    pub const MEDIUM_SPRING_GREEN: AnsiColor = AnsiColor(48);
    pub const MEDIUM_TURQUOISE: AnsiColor = AnsiColor(80);
    pub const MEDIUM_VIOLET_RED: AnsiColor = AnsiColor(162);
    pub const MIDNIGHT_BLUE: AnsiColor = AnsiColor(17);
    pub const MINT_CREAM: AnsiColor = AnsiColor(15);
    pub const MISTY_ROSE: AnsiColor = AnsiColor(224);
    pub const MOCCASIN: AnsiColor = AnsiColor(223);
    pub const NAVAJO_WHITE: AnsiColor = AnsiColor(223);
    pub const NAVY: AnsiColor = AnsiColor(4);
    pub const OLD_LACE: AnsiColor = AnsiColor(230);
    pub const OLIVE: AnsiColor = AnsiColor(3);
    pub const OLIVE_DRAB: AnsiColor = AnsiColor(64);
    pub const ORANGE: AnsiColor = AnsiColor(214);
    pub const ORANGE_RED: AnsiColor = AnsiColor(202);
    pub const ORCHID: AnsiColor = AnsiColor(170);
    pub const PALE_GOLDENROD: AnsiColor = AnsiColor(223);
    pub const PALE_GREEN: AnsiColor = AnsiColor(120);
    pub const PALE_TURQUOISE: AnsiColor = AnsiColor(159);
    pub const PALE_VIOLET_RED: AnsiColor = AnsiColor(168);
    pub const PAPAYA_WHIP: AnsiColor = AnsiColor(230);
    pub const PEACH_PUFF: AnsiColor = AnsiColor(223);
    pub const PERU: AnsiColor = AnsiColor(173);
    pub const PINK: AnsiColor = AnsiColor(218);
    pub const PLUM: AnsiColor = AnsiColor(182);
    pub const POWDER_BLUE: AnsiColor = AnsiColor(152);
    pub const PURPLE: AnsiColor = AnsiColor(5);
    pub const RED: AnsiColor = AnsiColor(9);
    pub const ROSY_BROWN: AnsiColor = AnsiColor(138);
    pub const ROYAL_BLUE: AnsiColor = AnsiColor(63);
    pub const SADDLE_BROWN: AnsiColor = AnsiColor(94);
    pub const SALMON: AnsiColor = AnsiColor(210);
    pub const SANDY_BROWN: AnsiColor = AnsiColor(215);
    pub const SEA_GREEN: AnsiColor = AnsiColor(29);
    pub const SEA_SHELL: AnsiColor = AnsiColor(15);
    pub const SIENNA: AnsiColor = AnsiColor(131);
    pub const SILVER: AnsiColor = AnsiColor(7);
    pub const SKY_BLUE: AnsiColor = AnsiColor(117);
    pub const SLATE_BLUE: AnsiColor = AnsiColor(62);
    pub const SLATE_GRAY: AnsiColor = AnsiColor(66);
    pub const SNOW: AnsiColor = AnsiColor(15);
    pub const SPRING_GREEN: AnsiColor = AnsiColor(48);
    pub const STEEL_BLUE: AnsiColor = AnsiColor(67);
    pub const TAN: AnsiColor = AnsiColor(180);
    pub const TEAL: AnsiColor = AnsiColor(6);
    pub const THISTLE: AnsiColor = AnsiColor(182);
    pub const TOMATO: AnsiColor = AnsiColor(203);
    pub const TURQUOISE: AnsiColor = AnsiColor(80);
    pub const VIOLET: AnsiColor = AnsiColor(213);
    pub const WHEAT: AnsiColor = AnsiColor(223);
    pub const WHITE: AnsiColor = AnsiColor(15);
    pub const WHITE_SMOKE: AnsiColor = AnsiColor(255);
    pub const YELLOW: AnsiColor = AnsiColor(11);
    pub const YELLOW_GREEN: AnsiColor = AnsiColor(149);

    // Mapping the Ansi Color name to color names provided in string.
    // Output will be Some(AnsiColor(0)) for "default".
    // To get the number, its AnsiColor::Default.code()
    pub fn get_ansi_color(color: &str) -> Option<AnsiColor> {
        match color {
            "default"               => Some(AnsiColor::DEFAULT),
            "aliceblue"             => Some(AnsiColor::ALICE_BLUE),
            "antiquewhite"          => Some(AnsiColor::ANTIQUE_WHITE),
            "aqua"                  => Some(AnsiColor::AQUA),
            "aquamarine"            => Some(AnsiColor::AQUAMARINE),
            "azure"                 => Some(AnsiColor::AZURE),
            "beige"                 => Some(AnsiColor::BEIGE),
            "bisque"                => Some(AnsiColor::BISQUE),
            "black"                 => Some(AnsiColor::BLACK),
            "blanchedalmond"        => Some(AnsiColor::BLANCHED_ALMOND),
            "blue"                  => Some(AnsiColor::BLUE),
            "blueviolet"            => Some(AnsiColor::BLUE_VIOLET),
            "brown"                 => Some(AnsiColor::BROWN),
            "burlywood"             => Some(AnsiColor::BURLY_WOOD),
            "cadetblue"             => Some(AnsiColor::CADET_BLUE),
            "chartreuse"            => Some(AnsiColor::CHARTREUSE),
            "chocolate"             => Some(AnsiColor::CHOCOLATE),
            "coral"                 => Some(AnsiColor::CORAL),
            "cornflowerblue"        => Some(AnsiColor::CORNFLOWER_BLUE),
            "cornsilk"              => Some(AnsiColor::CORNSILK),
            "crimson"               => Some(AnsiColor::CRIMSON),
            "cyan"                  => Some(AnsiColor::CYAN),
            "darkblue"              => Some(AnsiColor::DARK_BLUE),
            "darkcyan"              => Some(AnsiColor::DARK_CYAN),
            "darkgoldenrod"         => Some(AnsiColor::DARK_GOLDENROD),
            "darkgray"              => Some(AnsiColor::DARK_GRAY),
            "darkgreen"             => Some(AnsiColor::DARK_GREEN),
            "darkkhaki"             => Some(AnsiColor::DARK_KHAKI),
            "darkmagenta"           => Some(AnsiColor::DARK_MAGENTA),
            "darkolivegreen"        => Some(AnsiColor::DARK_OLIVE_GREEN),
            "darkorange"            => Some(AnsiColor::DARK_ORANGE),
            "darkorchid"            => Some(AnsiColor::DARK_ORCHID),
            "darkred"               => Some(AnsiColor::DARK_RED),
            "darksalmon"            => Some(AnsiColor::DARK_SALMON),
            "darkseagreen"          => Some(AnsiColor::DARK_SEA_GREEN),
            "darkslateblue"         => Some(AnsiColor::DARK_SLATE_BLUE),
            "darkslategray"         => Some(AnsiColor::DARK_SLATE_GRAY),
            "darkturquoise"         => Some(AnsiColor::DARK_TURQUOISE),
            "darkviolet"            => Some(AnsiColor::DARK_VIOLET),
            "deeppink"              => Some(AnsiColor::DEEP_PINK),
            "deepskyblue"           => Some(AnsiColor::DEEP_SKY_BLUE),
            "dimgray"               => Some(AnsiColor::DIM_GRAY),
            "dodgerblue"            => Some(AnsiColor::DODGER_BLUE),
            "firebrick"             => Some(AnsiColor::FIREBRICK),
            "floralwhite"           => Some(AnsiColor::FLORAL_WHITE),
            "forestgreen"           => Some(AnsiColor::FOREST_GREEN),
            "fuchsia"               => Some(AnsiColor::FUCHSIA),
            "gainsboro"             => Some(AnsiColor::GAINSBORO),
            "ghostwhite"            => Some(AnsiColor::GHOST_WHITE),
            "gold"                  => Some(AnsiColor::GOLD),
            "goldenrod"             => Some(AnsiColor::GOLDENROD),
            "gray"                  => Some(AnsiColor::GRAY),
            "green"                 => Some(AnsiColor::GREEN),
            "greenyellow"           => Some(AnsiColor::GREEN_YELLOW),
            "honeydew"              => Some(AnsiColor::HONEYDEW),
            "hotpink"               => Some(AnsiColor::HOT_PINK),
            "indianred"             => Some(AnsiColor::INDIAN_RED),
            "indigo"                => Some(AnsiColor::INDIGO),
            "ivory"                 => Some(AnsiColor::IVORY),
            "khaki"                 => Some(AnsiColor::KHAKI),
            "lavender"              => Some(AnsiColor::LAVENDER),
            "lavenderblush"         => Some(AnsiColor::LAVENDER_BLUSH),
            "lawngreen"             => Some(AnsiColor::LAWN_GREEN),
            "lemonchiffon"          => Some(AnsiColor::LEMON_CHIFFON),
            "lightblue"             => Some(AnsiColor::LIGHT_BLUE),
            "lightcoral"            => Some(AnsiColor::LIGHT_CORAL),
            "lightcyan"             => Some(AnsiColor::LIGHT_CYAN),
            "lightgoldenrodyellow"  => Some(AnsiColor::LIGHT_GOLDENROD_YELLOW),
            "lightgray"             => Some(AnsiColor::LIGHT_GRAY),
            "lightgreen"            => Some(AnsiColor::LIGHT_GREEN),
            "lightpink"             => Some(AnsiColor::LIGHT_PINK),
            "lightsalmon"           => Some(AnsiColor::LIGHT_SALMON),
            "lightseagreen"         => Some(AnsiColor::LIGHT_SEA_GREEN),
            "lightskyblue"          => Some(AnsiColor::LIGHT_SKY_BLUE),
            "lightslategray"        => Some(AnsiColor::LIGHT_SLATE_GRAY),
            "lightsteelblue"        => Some(AnsiColor::LIGHT_STEEL_BLUE),
            "lightyellow"           => Some(AnsiColor::LIGHT_YELLOW),
            "lime"                  => Some(AnsiColor::LIME),
            "limegreen"             => Some(AnsiColor::LIME_GREEN),
            "linen"                 => Some(AnsiColor::LINEN),
            "magenta"               => Some(AnsiColor::MAGENTA),
            "maroon"                => Some(AnsiColor::MAROON),
            "mediumaquamarine"      => Some(AnsiColor::MEDIUM_AQUAMARINE),
            "mediumblue"            => Some(AnsiColor::MEDIUM_BLUE),
            "mediumorchid"          => Some(AnsiColor::MEDIUM_ORCHID),
            "mediumpurple"          => Some(AnsiColor::MEDIUM_PURPLE),
            "mediumseagreen"        => Some(AnsiColor::MEDIUM_SEA_GREEN),
            "mediumslateblue"       => Some(AnsiColor::MEDIUM_SLATE_BLUE),
            "mediumspringgreen"     => Some(AnsiColor::MEDIUM_SPRING_GREEN),
            "mediumturquoise"       => Some(AnsiColor::MEDIUM_TURQUOISE),
            "mediumvioletred"       => Some(AnsiColor::MEDIUM_VIOLET_RED),
            "midnightblue"          => Some(AnsiColor::MIDNIGHT_BLUE),
            "mintcream"             => Some(AnsiColor::MINT_CREAM),
            "mistyrose"             => Some(AnsiColor::MISTY_ROSE),
            "moccasin"              => Some(AnsiColor::MOCCASIN),
            "navajowhite"           => Some(AnsiColor::NAVAJO_WHITE),
            "navy"                  => Some(AnsiColor::NAVY),
            "oldlace"               => Some(AnsiColor::OLD_LACE),
            "olive"                 => Some(AnsiColor::OLIVE),
            "olivedrab"             => Some(AnsiColor::OLIVE_DRAB),
            "orange"                => Some(AnsiColor::ORANGE),
            "orangered"             => Some(AnsiColor::ORANGE_RED),
            "orchid"                => Some(AnsiColor::ORCHID),
            "palegoldenrod"         => Some(AnsiColor::PALE_GOLDENROD),
            "palegreen"             => Some(AnsiColor::PALE_GREEN),
            "paleturquoise"         => Some(AnsiColor::PALE_TURQUOISE),
            "palevioletred"         => Some(AnsiColor::PALE_VIOLET_RED),
            "papayawhip"            => Some(AnsiColor::PAPAYA_WHIP),
            "peachpuff"             => Some(AnsiColor::PEACH_PUFF),
            "peru"                  => Some(AnsiColor::PERU),
            "pink"                  => Some(AnsiColor::PINK),
            "plum"                  => Some(AnsiColor::PLUM),
            "powderblue"            => Some(AnsiColor::POWDER_BLUE),
            "purple"                => Some(AnsiColor::PURPLE),
            "red"                   => Some(AnsiColor::RED),
            "rosybrown"             => Some(AnsiColor::ROSY_BROWN),
            "royalblue"             => Some(AnsiColor::ROYAL_BLUE),
            "saddlebrown"           => Some(AnsiColor::SADDLE_BROWN),
            "salmon"                => Some(AnsiColor::SALMON),
            "sandybrown"            => Some(AnsiColor::SANDY_BROWN),
            "seagreen"              => Some(AnsiColor::SEA_GREEN),
            "seashell"              => Some(AnsiColor::SEA_SHELL),
            "sienna"                => Some(AnsiColor::SIENNA),
            "silver"                => Some(AnsiColor::SILVER),
            "skyblue"               => Some(AnsiColor::SKY_BLUE),
            "slateblue"             => Some(AnsiColor::SLATE_BLUE),
            "slategray"             => Some(AnsiColor::SLATE_GRAY),
            "snow"                  => Some(AnsiColor::SNOW),
            "springgreen"           => Some(AnsiColor::SPRING_GREEN),
            "steelblue"             => Some(AnsiColor::STEEL_BLUE),
            "tan"                   => Some(AnsiColor::TAN),
            "teal"                  => Some(AnsiColor::TEAL),
            "thistle"               => Some(AnsiColor::THISTLE),
            "tomato"                => Some(AnsiColor::TOMATO),
            "turquoise"             => Some(AnsiColor::TURQUOISE),
            "violet"                => Some(AnsiColor::VIOLET),
            "wheat"                 => Some(AnsiColor::WHEAT),
            "white"                 => Some(AnsiColor::WHITE),
            "whitesmoke"            => Some(AnsiColor::WHITE_SMOKE),
            "yellow"                => Some(AnsiColor::YELLOW),
            "yellowgreen"           => Some(AnsiColor::YELLOW_GREEN),
            _                       => None,
        }
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

        let c = if *self == AnsiColor::BLACK { 0u8 } else { self.0 };

        if c <= AnsiColor::SILVER.into() {
            write!(f, "\x1b[{}m", 30 + c)
        } else if c <= AnsiColor::WHITE.into() {
            write!(f, "\x1b[{}m", 82 + c)
        } else {
            write!(f, "\x1b[38;5;{}m", c)
        }
    }
}

