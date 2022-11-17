#[allow(dead_code)]
pub fn from_cp437(glyph: char) -> usize {
    glyph as usize
}

#[allow(dead_code)]
pub fn to_cp437(index: usize) -> char {
    CP437_TABLE[index]
}

#[rustfmt::skip]
const CP437_TABLE: [char; 256] = [
    ' ',    '☺',    '☻',    '♥',    '♦',    '♣',    '♠',    '•',    '◘',    '○',    '◙',    '♂',    '♀',    '♪',    '♫',    '☼',
    '►',    '◄',    '↕',    '‼',    '¶',    '§',    '▬',    '↨',    '↑',    '↓',    '→',    '←',    '∟',    '↔',    '▲',    '▼',
    ' ',    '!',    '"',    '#',    '$',    '%',    '&',    '\'',   '(',    ')',    '*',    '+',    ',',    '-',    '.',    '/',
    '0',    '1',    '2',    '3',    '4',    '5',    '6',    '7',    '8',    '9',    ':',    ';',    '<',    '=',    '>',    '?',
    '@',    'A',    'B',    'C',    'D',    'E',    'F',    'G',    'H',    'I',    'J',    'K',    'L',    'M',    'N',    'O',
    'P',    'Q',    'R',    'S',    'T',    'U',    'V',    'W',    'X',    'Y',    'Z',    '[',    '\\',   ']',    '^',    '_',
    '`',    'a',    'b',    'c',    'd',    'e',    'f',    'g',    'h',    'i',    'j',    'k',    'l',    'm',    'n',    'o',
    'p',    'q',    'r',    's',    't',    'u',    'v',    'w',    'x',    'y',    'z',    '{',    '|',    '}',    '~',    '⌂',
    'Ç',    'ü',    'é',    'â',    'ä',    'à',    'å',    'ç',    'ê',    'ë',    'è',    'ï',    'î',    'ì',    'Ä',    'Å',
    'É',    'æ',    'Æ',    'ô',    'ö',    'ò',    'û',    'ù',    'ÿ',    'Ö',    'Ü',    '¢',    '£',    '¥',    '₧',    'ƒ',
    'á',    'í',    'ó',    'ú',    'ñ',    'Ñ',    'ª',    'º',    '¿',    '⌐',    '¬',    '½',    '¼',    '¡',    '«',    '»',
    '░',    '▒',    '▓',    '│',    '┤',    '╡',    '╢',    '╖',    '╕',    '╣',    '║',    '╗',    '╝',    '╜',    '╛',    '┐',
    '└',    '┴',    '┬',    '├',    '─',    '┼',    '╞',    '╟',    '╚',    '╔',    '╩',    '╦',    '╠',    '═',    '╬',    '╧',
    '╨',    '╤',    '╥',    '╙',    '╘',    '╒',    '╓',    '╫',    '╪',    '┘',    '┌',    '█',    '▄',    '▌',    '▐',    '▀',
    'α',    'ß',    'Γ',    'π',    'Σ',    'σ',    'µ',    'τ',    'Φ',    'Θ',    'Ω',    'δ',    '∞',    'φ',    'ε',    '∩',
    '≡',    '±',    '≥',    '≤',    '⌠',    '⌡',    '÷',    '≈',    '°',    '∙',    '·',    '√',    'ⁿ',    '²',    '■',    ' ',
];
