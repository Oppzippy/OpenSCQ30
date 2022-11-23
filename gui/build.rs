fn main() {
    glib_build_tools::compile_resources(
        "src/volume_slider",
        "src/volume_slider/resources.gresource.xml",
        "volume_slider.gresource",
    );
    glib_build_tools::compile_resources(
        "src/equalizer",
        "src/equalizer/resources.gresource.xml",
        "equalizer.gresource",
    );
}
