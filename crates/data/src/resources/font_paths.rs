use crate::prelude::*;

/// An ID used to identify a [`Font`]
pub type FontId = u8;

/////////////////////////////////////////////////////////////////////
/// INTERNAL FILE STRUCTURE
/////////////////////////////////////////////////////////////////////

// Font Extension
const RON_EXT: &str = ".ron";

// Tileset definitions folder
const DEFINITIONS_FOLDER: &str = "fonts/definitions";

/////////////////////////////////////////////////////////////////////
/// FONT IDS
/////////////////////////////////////////////////////////////////////

// Tileset Names (Match filename to infile name)
#[cfg(feature = "debug")] // Show TODO to catch bugs in debug mod
pub const FONT_MISSING_NAME: &str = "missing_debug";

#[cfg(not(feature = "debug"))] // Show invisible for release builds
pub const FONT_MISSING_NAME: &str = "missing";
pub const FONT_MISSING_ID: &FontId = &0;

/////////////////////////////////////////////////////////////////////
/// FUNCTIONS
/////////////////////////////////////////////////////////////////////

/// This function is used during tileset loading
pub fn get_font_paths() -> Vec<String,> {
    let mut ret = Vec::new();

    // add the "missing" tileset
    ret.push(format!("{DEFINITIONS_FOLDER}{TILESET_MISSING_NAME}{RON_EXT}"),);

    ret
}
