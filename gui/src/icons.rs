use cosmic::widget::icon::Handle;

pub fn openscq30() -> Handle {
    cosmic::widget::icon::from_svg_bytes(include_bytes!("../resources/com.oppzippy.OpenSCQ30.svg"))
        .symbolic(false)
}
