use crate::common::*;

const VORTEX: u8 = 1;

/// Cartesian:      ψ = C A |y|² (a² - x² - |y|²)
/// Cylindrical:    ψ = C A (a² - r²) r² sin²θ
/// domain -2 < x < 2 and -2 < y < 2
pub fn hills() -> u8 {
    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
