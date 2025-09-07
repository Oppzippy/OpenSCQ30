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
pub(crate) use include_icon;
