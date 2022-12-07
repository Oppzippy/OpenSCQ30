fn main() {
    glib_build_tools::compile_resources(
        "src/widgets",
        "src/widgets/widgets.gresource.xml",
        "widgets.gresource",
    );
}
