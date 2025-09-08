use cosmic::widget::icon::Handle;

macro_rules! include_icon {
    ($name:tt, $path:tt $(,)?) => {{
        #[cfg(target_os = "linux")]
        {
            ::cosmic::widget::icon::from_name($name).handle()
        }
        #[cfg(not(target_os = "linux"))]
        {
            ::cosmic::widget::icon::from_svg_bytes(::std::include_bytes!($path)).symbolic(true)
        }
    }};
}

pub fn openscq30() -> Handle {
    cosmic::widget::icon::from_svg_bytes(include_bytes!("../resources/com.oppzippy.OpenSCQ30.svg"))
        .symbolic(false)
}

pub fn dialog_warning_symbolic() -> Handle {
    include_icon!(
        "dialog-warning-symbolic",
        "../icons/dialog-warning-symbolic.svg"
    )
}

pub fn edit_copy_symbolic() -> Handle {
    include_icon!("edit-copy-symbolic", "../icons/edit-copy-symbolic.svg")
}

pub fn go_previous_symbolic() -> Handle {
    include_icon!("go-previous-symbolic", "../icons/go-previous-symbolic.svg")
}

pub fn list_add_symbolic() -> Handle {
    include_icon!("list-add-symbolic", "../icons/list-add-symbolic.svg")
}

pub fn list_remove_symbolic() -> Handle {
    include_icon!("list-remove-symbolic", "../icons/list-remove-symbolic.svg")
}

pub fn view_refresh_symbolic() -> Handle {
    include_icon!(
        "view-refresh-symbolic",
        "../icons/view-refresh-symbolic.svg"
    )
}

pub fn help_about_symbolic() -> Handle {
    include_icon!("help-about-symbolic", "../icons/help-about-symbolic.svg")
}
