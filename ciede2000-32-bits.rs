// This function written in Rust is not affiliated with the CIE (International Commission on Illumination),
// and is released into the public domain. It is provided "as is" without any warranty, express or implied.

// The classic CIE ΔE2000 implementation, which operates on two L*a*b* colors, and returns their difference.
// "l" ranges from 0 to 100, while "a" and "b" are unbounded and commonly clamped to the range of -128 to 127.
fn ciede2000(l1: f32, a1: f32, b1: f32, l2: f32, a2: f32, b2: f32, kl: f32, kc: f32, kh: f32, canonical: bool) -> f32 {
	// Working in Rust with the CIEDE2000 color-difference formula.
	// kl, kc, kh are parametric factors to be adjusted according to
	// different viewing parameters such as textures, backgrounds...
	const M_PI: f32 = std::f32::consts::PI;
	let mut n = ((a1 * a1 + b1 * b1).sqrt() + (a2 * a2 + b2 * b2).sqrt()) * 0.5f32;
	n = n * n * n * n * n * n * n;
	// A factor involving chroma raised to the power of 7 designed to make
	// the influence of chroma on the total color difference more accurate.
	n = 1.0f32 + 0.5f32 * (1.0f32 - (n / (n + 6103515625.0f32)).sqrt());
	// Application of the chroma correction factor.
	let c1: f32 = (a1 * a1 * n * n + b1 * b1).sqrt();
	let c2: f32 = (a2 * a2 * n * n + b2 * b2).sqrt();
	// atan2 is preferred over atan because it accurately computes the angle of
	// a point (x, y) in all quadrants, handling the signs of both coordinates.
	let mut h1 = b1.atan2(a1 * n);
	let mut h2 = b2.atan2(a2 * n);
	if h1 < 0.0f32 { h1 += 2.0f32 * M_PI; }
	if h2 < 0.0f32 { h2 += 2.0f32 * M_PI; }
	// When the hue angles lie in different quadrants, the straightforward
	// average can produce a mean that incorrectly suggests a hue angle in
	// the wrong quadrant, the next lines handle this issue.
	let mut h_mean = (h1 + h2) * 0.5f32;
	let mut h_delta = (h2 - h1) * 0.5f32;
	if M_PI + 0.000001f32 < (h2 - h1).abs() {
		h_delta += M_PI;
		if canonical && M_PI + 0.000001f32 < h_mean {
			// Sharma’s implementation, OpenJDK, ...
			h_mean -= M_PI;
		} else {
			// Lindbloom’s implementation, Netflix’s VMAF, ...
			h_mean += M_PI;
		}
	}
	let p = 36.0f32 * h_mean - 55.0f32 * M_PI;
	n = (c1 + c2) * 0.5f32;
	n = n * n * n * n * n * n * n;
	// The hue rotation correction term is designed to account for the
	// non-linear behavior of hue differences in the blue region.
	let r_t = -2.0f32 * (n / (n + 6103515625.0f32)).sqrt()
		* (M_PI / 3.0f32 * (p * p / (-25.0f32 * M_PI * M_PI)).exp()).sin();
	n = (l1 + l2) * 0.5f32;
	n = (n - 50.0f32) * (n - 50.0f32);
	// Lightness.
	let l = (l2 - l1) / (kl * (1.0f32 + 0.015f32 * n / (20.0f32 + n).sqrt()));
	// These coefficients adjust the impact of different harmonic
	// components on the hue difference calculation.
	let t = 1.0f32	- 0.17f32 * (h_mean + M_PI / 3.0f32).sin()
					+ 0.24f32 * (2.0f32 * h_mean + M_PI * 0.5f32).sin()
					+ 0.32f32 * (3.0f32 * h_mean + 8.0f32 * M_PI / 15.0f32).sin()
					- 0.20f32 * (4.0f32 * h_mean + 3.0f32 * M_PI / 20.0f32).sin();
	n = c1 + c2;
	// Hue.
	let h = 2.0f32 * (c1 * c2).sqrt() * (h_delta).sin()
		/ (kh * (1.0f32 + 0.0075f32 * n * t));
	// Chroma.
	let c = (c2 - c1) / (kc * (1.0f32 + 0.0225f32 * n));
	// The result reflects the actual geometric distance in the color space, given a tolerance of 0.00022.
	(l * l + h * h + c * c + c * h * r_t).sqrt()
}

// If you remove the constant 0.000001, the code will continue to work, but CIEDE2000
// interoperability between all programming languages will no longer be guaranteed.
