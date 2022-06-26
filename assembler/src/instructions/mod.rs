mod btype; 
mod itype;
mod jtype;
mod rtype;
mod stype;
mod utype;
mod types;

/// Translates an instruction into binary
pub trait Translate {
    fn translate(&self) -> Vec<u8>;
}

enum Instructions {
    R(rtype::RType),
    I(itype::IType),
    S(stype::SType),
    B(btype::BType),
    U(utype::UType),
    J(jtype::JType),
}

