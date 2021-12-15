pub(crate) struct Scene {
    x_scroll_offset: u32,
    y_scroll_offset: u32,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            x_scroll_offset: 0,
            y_scroll_offset: 0,
        }
    }
}

impl Scene {
    pub(crate) fn init_first_stage() -> Self {
        Self { ..Self::default() }
    }
}
