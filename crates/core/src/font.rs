//! Font scaling helpers (pure, no GPUI).

/// Window rem size in logical pixels for a saved `font_size_scale`.
/// The scale is clamped to 0.8..=1.3 around a 16 px base.
pub fn rem_px(scale: f32) -> f32 {
    16.0 * scale.clamp(0.8, 1.3)
}

#[cfg(test)]
mod tests {
    use super::rem_px;

    #[test]
    fn font_rem_px_scales_and_clamps() {
        assert!((rem_px(0.8) - 12.8).abs() < 1e-4);
        assert!((rem_px(1.3) - 20.8).abs() < 1e-4);
        assert!((rem_px(2.0) - 20.8).abs() < 1e-4);
    }
}
