pub trait Translate {
    // Translates an instruction into binary
    fn translate(&self) -> Vec<u8>;
}
