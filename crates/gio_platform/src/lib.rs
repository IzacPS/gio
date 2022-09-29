pub mod common;
pub mod input;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub mod window {
    pub type Window<'a> = super::linux::window::Window<'a>;
    pub type WindowInterface = super::linux::window::WindowInterface;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_create_window() {}
}
