//! P3 (ASCII) PPM image writer (RT-010).

use crate::vec3::Color;
use std::io::{self, Write};

/// Write a P3 PPM image: header `P3` / `width height` / `255`, then one `R G B` per pixel.
///
/// Pixels are row-major, top-left → bottom-right (same order as [`crate::tracer::render_frame`]).
pub fn write_ppm_p3<W: Write>(
    out: &mut W,
    width: u32,
    height: u32,
    pixels: &[Color],
) -> io::Result<()> {
    let expected = (width as usize).saturating_mul(height as usize);
    assert_eq!(
        pixels.len(),
        expected,
        "pixel buffer length {} != width*height {}",
        pixels.len(),
        expected
    );

    writeln!(out, "P3")?;
    writeln!(out, "{width} {height}")?;
    writeln!(out, "255")?;

    for color in pixels {
        let (r, g, b) = color.to_rgb8();
        writeln!(out, "{r} {g} {b}")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_and_one_pixel() {
        let pixels = [Color::new(1.0, 0.0, 0.5)];
        let mut buf = Vec::new();
        write_ppm_p3(&mut buf, 1, 1, &pixels).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(s, "P3\n1 1\n255\n255 0 128\n");
    }

    #[test]
    fn row_major_two_by_one() {
        let pixels = [
            Color::new(1.0, 0.0, 0.0),
            Color::new(0.0, 1.0, 0.0),
        ];
        let mut buf = Vec::new();
        write_ppm_p3(&mut buf, 2, 1, &pixels).unwrap();
        let s = String::from_utf8(buf).unwrap();
        let lines: Vec<_> = s.lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "2 1");
        assert_eq!(lines[2], "255");
        assert_eq!(lines[3], "255 0 0");
        assert_eq!(lines[4], "0 255 0");
    }

    #[test]
    #[should_panic]
    fn rejects_wrong_buffer_len() {
        let pixels = [Color::BLACK];
        let mut buf = Vec::new();
        let _ = write_ppm_p3(&mut buf, 2, 2, &pixels);
    }
}
