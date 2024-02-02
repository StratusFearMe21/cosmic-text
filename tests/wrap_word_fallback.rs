use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, Wrap};

// Tests the ability to fallback to glyph wrapping when a word can't fit on a line by itself.
// No line should ever overflow the buffer size.
#[test]
fn wrap_word_fallback() {
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(14.0, 20.0);

    let mut buffer = Buffer::new(&mut font_system, metrics);

    let mut buffer = buffer.borrow_with(&mut font_system);

    buffer.set_wrap(Wrap::WordOrGlyph);
    buffer.set_text("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.", Attrs::new().family(cosmic_text::Family::Name("Inter")), Shaping::Advanced);
    buffer.set_size(50.0, 1000.0);

    buffer.shape_until_scroll(false);

    let measured_size = measure(&buffer);

    assert!(
        measured_size <= buffer.size().0,
        "Measured width is larger than buffer width\n{} <= {}",
        measured_size,
        buffer.size().0
    );
}

fn measure(buffer: &Buffer) -> f32 {
    buffer
        .layout_runs()
        .fold(0.0f32, |width, run| width.max(run.line_w))
}
