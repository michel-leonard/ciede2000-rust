// This function written in Rust is not affiliated with the CIE (International Commission on Illumination),
// and is released into the public domain. It is provided "as is" without any warranty, express or implied.

// Convenience function, with parametric factors set to their default values.
fn ciede2000(l1: f64, a1: f64, b1: f64, l2: f64, a2: f64, b2: f64) -> f64 {
    ciede2000_with_parameters(l1, a1, b1, l2, a2, b2, 1.0, 1.0, 1.0, false)
}

// The classic CIE ΔE2000 implementation, which operates on two L*a*b* colors, and returns their difference.
// "l" ranges from 0 to 100, while "a" and "b" are unbounded and commonly clamped to the range of -128 to 127.
fn ciede2000_with_parameters(l1: f64, a1: f64, b1: f64, l2: f64, a2: f64, b2: f64, kl: f64, kc: f64, kh: f64, canonical: bool) -> f64 {
	// Working in Rust with the CIEDE2000 color-difference formula.
	// kl, kc, kh are parametric factors to be adjusted according to
	// different viewing parameters such as textures, backgrounds...
	const M_PI: f64 = std::f64::consts::PI;
	let mut n = ((a1 * a1 + b1 * b1).sqrt() + (a2 * a2 + b2 * b2).sqrt()) * 0.5;
	n = n * n * n * n * n * n * n;
	// A factor involving chroma raised to the power of 7 designed to make
	// the influence of chroma on the total color difference more accurate.
	n = 1.0 + 0.5 * (1.0 - (n / (n + 6103515625.0)).sqrt());
	// Application of the chroma correction factor.
	let c1: f64 = (a1 * a1 * n * n + b1 * b1).sqrt();
	let c2: f64 = (a2 * a2 * n * n + b2 * b2).sqrt();
	// atan2 is preferred over atan because it accurately computes the angle of
	// a point (x, y) in all quadrants, handling the signs of both coordinates.
	let mut h1 = b1.atan2(a1 * n);
	let mut h2 = b2.atan2(a2 * n);
	if h1 < 0.0 { h1 += 2.0 * M_PI; }
	if h2 < 0.0 { h2 += 2.0 * M_PI; }
	// When the hue angles lie in different quadrants, the straightforward
	// average can produce a mean that incorrectly suggests a hue angle in
	// the wrong quadrant, the next lines handle this issue.
	let mut h_mean = (h1 + h2) * 0.5;
	let mut h_delta = (h2 - h1) * 0.5;
	if M_PI + 1E-14 < (h2 - h1).abs() {
		h_delta += M_PI;
		if canonical && M_PI + 1E-14 < h_mean {
			// Sharma’s implementation, OpenJDK, ...
			h_mean -= M_PI;
		} else {
			// Lindbloom’s implementation, Netflix’s VMAF, ...
			h_mean += M_PI;
		}
	}
	let p = 36.0 * h_mean - 55.0 * M_PI;
	n = (c1 + c2) * 0.5;
	n = n * n * n * n * n * n * n;
	// The hue rotation correction term is designed to account for the
	// non-linear behavior of hue differences in the blue region.
	let r_t = -2.0 * (n / (n + 6103515625.0)).sqrt()
		* (M_PI / 3.0 * (p * p / (-25.0 * M_PI * M_PI)).exp()).sin();
	n = (l1 + l2) * 0.5;
	n = (n - 50.0) * (n - 50.0);
	// Lightness.
	let l = (l2 - l1) / (kl * (1.0 + 0.015 * n / (20.0 + n).sqrt()));
	// These coefficients adjust the impact of different harmonic
	// components on the hue difference calculation.
	let t = 1.0	- 0.17 * (h_mean + M_PI / 3.0).sin()
				+ 0.24 * (2.0 * h_mean + M_PI * 0.5).sin()
				+ 0.32 * (3.0 * h_mean + 8.0 * M_PI / 15.0).sin()
				- 0.20 * (4.0 * h_mean + 3.0 * M_PI / 20.0).sin();
	n = c1 + c2;
	// Hue.
	let h = 2.0 * (c1 * c2).sqrt() * (h_delta).sin() / (kh * (1.0 + 0.0075 * n * t));
	// Chroma.
	let c = (c2 - c1) / (kc * (1.0 + 0.0225 * n));
	// The result reflects the actual geometric distance in the color space, given a tolerance of 3.4e-13.
	(l * l + h * h + c * c + c * h * r_t).sqrt()
}

// If you remove the constant 1E-14, the code will continue to work, but CIEDE2000
// interoperability between all programming languages will no longer be guaranteed.

// Source code tested by Michel LEONARD
// Website: ciede2000.pages-perso.free.fr
