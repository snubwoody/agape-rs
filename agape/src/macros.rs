/// Implement common styling attributes
#[macro_export]
macro_rules! impl_style {
    () => {
        /// Set the background [`Color`] of a [`Widget`].
        pub fn background_color(mut self, color: impl $crate::IntoColor<$crate::Rgba>) -> Self {
            self.style.background_color = color.into_color();
            self
        }

        /// Set the widgets border width.
        pub fn border_width(mut self, width: f32) -> Self {
            self.style.border_width(width);
            self
        }

        /// Set the widgets border [`Color`].
        pub fn border_color(mut self, color: impl $crate::IntoColor<$crate::Rgba>) -> Self {
            self.style.border_color(color);
            self
        }

        /// Add a border.
        pub fn border(mut self, border: $crate::style::Border) -> Self {
            self.style.border = Some(border);
            self
        }

        /// Make the [`Widget`] fit it's children.
        pub fn fit(mut self) -> Self {
            self.style.fit();
            self
        }

        pub fn fit_height(mut self) -> Self {
            self.style.fit_height();
            self
        }

        pub fn fit_width(mut self) -> Self {
            self.style.fit_width();
            self
        }

        /// Make the widget fill its parent.
        pub fn fill(mut self) -> Self {
            self.style.fill();
            self
        }

        pub fn fill_width(mut self) -> Self {
            self.style.fill_width();
            self
        }

        pub fn fill_height(mut self) -> Self {
            self.style.fill_height();
            self
        }

        /// Make the [`Widget`] fit it's children.
        pub fn fixed(mut self, width: f32, height: f32) -> Self {
            self.style.fixed(width, height);
            self
        }
    };
}
