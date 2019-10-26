use crate::common::*;

const VORTEX: u8 = 1;

/// Cartesian:      ψ = C A |y|² (a² - x² - |y|²)
/// Cylindrical:    ψ = C A (a² - r²) r² sin²θ
/// domain -2 < x < 2 and -2 < y < 2
pub fn hills(m: usize, n: usize) -> Vec<usize> {
    // radius
    let a = 1;
    let u0 = 10;
    let A = 15/2 * u0 * 1/a^2;

    let mut z = Vec::with_capacity(( m * n) as usize);

    for i in 0..m {
        for j in 0..n {
            // let u_int = 1/5 * A * (a^2 - i^2 - 2 * j^2);
            let v_int = 1/5 * A * i * j^2;
            z.push(v_int);
        }
    }
    z
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generates_hills_spherical_vortex() {
        assert_eq!(hills(), 1);
    }
}
