use cosmic::widget::icon::Handle;

macro_rules! icon {
    ($($icon_name:tt as $function_name:ident),+ $(,)?) => {
        $(
            pub fn $function_name() -> Handle {
                #[cfg(target_os = "linux")]
                {
                    ::cosmic::widget::icon::from_name($icon_name).handle()
                }
                #[cfg(not(target_os = "linux"))]
                {
                    ::cosmic::widget::icon::from_svg_bytes(::std::include_bytes!(concat!(
                        "../icons/",
                        $icon_name,
                        ".svg"
                    )))
                    .symbolic(true)
                }
            }
        )+
    };
}

pub fn openscq30() -> Handle {
    cosmic::widget::icon::from_svg_bytes(include_bytes!("../resources/com.oppzippy.OpenSCQ30.svg"))
        .symbolic(false)
}

icon!(
    "dialog-warning-symbolic" as dialog_warning_symbolic,
    "edit-copy-symbolic" as edit_copy_symbolic,
    "go-previous-symbolic" as go_previous_symbolic,
    "list-add-symbolic" as list_add_symbolic,
    "list-remove-symbolic" as list_remove_symbolic,
    "view-refresh-symbolic" as view_refresh_symbolic,
    "help-about-symbolic" as help_about_symbolic,
);
