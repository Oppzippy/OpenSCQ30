use rifgen::rifgen_attr::*;

mod java_glue;
pub use crate::java_glue::*;

struct HelloWorldRust {}

impl HelloWorldRust {
    #[generate_interface(constructor)]
    pub fn new() -> HelloWorldRust {
        Self {}
    }

    #[generate_interface]
    pub fn greet(&self, to: &str) -> String {
        format!("Hello {to}")
    }
}
