Browse : [PHP](https://github.com/michel-leonard/ciede2000-php) · [Perl](https://github.com/michel-leonard/ciede2000-perl) · [Python](https://github.com/michel-leonard/ciede2000-python) · [R](https://github.com/michel-leonard/ciede2000-r) · [Ruby](https://github.com/michel-leonard/ciede2000-ruby) · **Rust** · [SQL](https://github.com/michel-leonard/ciede2000-sql) · [Swift](https://github.com/michel-leonard/ciede2000-swift) · [TypeScript](https://github.com/michel-leonard/ciede2000-typescript) · [VBA](https://github.com/michel-leonard/ciede2000-vba) · [Wolfram Language](https://github.com/michel-leonard/ciede2000-wolfram-language)

# CIEDE2000 color difference formula in Rust

This page presents the CIEDE2000 color difference, implemented in the Rust programming language.

![Logo](https://raw.githubusercontent.com/michel-leonard/ciede2000-color-matching/refs/heads/main/docs/assets/images/logo.jpg)

## About

Here you’ll find the first rigorously correct implementation of CIEDE2000 that doesn’t use any conversion between degrees and radians. Set parameter `canonical` to obtain results in line with your existing pipeline.

`canonical`|The algorithm operates...|
|:--:|-|
`false`|in accordance with the CIEDE2000 values currently used by many industry players|
`true`|in accordance with the CIEDE2000 values provided by [this](https://hajim.rochester.edu/ece/sites/gsharma/ciede2000/) academic MATLAB function|

## Our CIEDE2000 offer

These 2 production-ready files, released in 2026, contain the CIEDE2000 algorithm.

Source File|Type|Bits|Purpose|Advantage|
|:--:|:--:|:--:|:--:|:--:|
[ciede2000-32-bits.rs](./ciede2000-32-bits.rs)|`f32`|32|General|Lightness, Speed|
[ciede2000-64-bits.rs](./ciede2000-64-bits.rs)|`f64`|64|Scientific|Interoperability|

### Software Versions

- rustc 1.94

### Example Usage

We compute the CIEDE2000 distance between two L\*a\*b\* colors, not specifying the optional parameters.

```rust
// ciede2000-64-bits.rs must be included here.

fn main() {
	let delta_e = ciede2000(95.3, 58.8, 2.1, 95.7, 61.9, -1.7);
	println!("CIEDE2000 = {}", delta_e); // ΔE2000 = 1.940859230419468
}
```

When the last 4 parameters must change, you can use `ciede2000_with_parameters` as follows.

```rust
// ciede2000-64-bits.rs must be included here.

fn main() {
	let delta_e = ciede2000_with_parameters(98.8, 71.4, 6.4, 93.9, 43.4, -3.3, 1.0, 1.1, 0.9, true);
	println!("CIEDE2000 = {}", delta_e); // ΔE2000 = 9.066556776527793
}
```

These CIEDE2000 calculations in Rust are fast, typically allowing millions of color comparisons per second.

### Test Results

LEONARD’s tests are based on well-chosen L\*a\*b\* colors, with various parametric factors `kL`, `kC` and `kH`.

<details>
<summary>Display test results for 3 correct decimal places in 32-bits</summary>

```
CIEDE2000 Verification Summary :
          Compliance : [ ] CANONICAL [X] SIMPLIFIED
  First Checked Line : 40.0,0.5,-128.0,49.91,0.0,24.0,1.0,1.0,1.0,51.01867
           Precision : 3 decimal digits
           Successes : 10000000
               Error : 0
            Duration : 23.17 seconds
     Average Delta E : 63.58
   Average Deviation : 7.6e-06
   Maximum Deviation : 0.00016
```

```
CIEDE2000 Verification Summary :
          Compliance : [X] CANONICAL [ ] SIMPLIFIED
  First Checked Line : 40.0,0.5,-128.0,49.91,0.0,24.0,1.0,1.0,1.0,51.018467
           Precision : 3 decimal digits
           Successes : 10000000
               Error : 0
            Duration : 23.18 seconds
     Average Delta E : 63.58
   Average Deviation : 7.2e-06
   Maximum Deviation : 0.00019
```

</details>

<details>
<summary>Display test results for 12 correct decimal places in 64-bits</summary>

```
CIEDE2000 Verification Summary :
          Compliance : [ ] CANONICAL [X] SIMPLIFIED
  First Checked Line : 40.0,0.5,-128.0,49.91,0.0,24.0,1.0,1.0,1.0,51.01866090771252
           Precision : 12 decimal digits
           Successes : 100000000
               Error : 0
            Duration : 260.26 seconds
     Average Delta E : 67.13
   Average Deviation : 4.2e-15
   Maximum Deviation : 1.4e-13
```

```
CIEDE2000 Verification Summary :
          Compliance : [X] CANONICAL [ ] SIMPLIFIED
  First Checked Line : 40.0,0.5,-128.0,49.91,0.0,24.0,1.0,1.0,1.0,51.018463019698125
           Precision : 12 decimal digits
           Successes : 100000000
               Error : 0
            Duration : 262.43 seconds
     Average Delta E : 67.13
   Average Deviation : 4.7e-15
   Maximum Deviation : 1.4e-13
```

</details>

## Public Domain Licence

You are free to use these files, even for commercial purposes.
