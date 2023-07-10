// make sub::private only visible to sibling modules
mod private;
// make sub::public visible as public
pub mod public;
// not adding sub::hidden on purpose to hide it from siblings
