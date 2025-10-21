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
        pub fn border(mut self, border: agape_core::Border) -> Self {
            self.style.border = Some(border);
            self
        }

        /// Set the corner radius.
        pub fn corner_radius(mut self, radius: u32) -> Self {
            self.style.corner_radius(radius);
            self
        }

        pub fn padding(mut self, padding: $crate::Padding) -> Self {
            self.style.padding(padding);
            self
        }

        pub fn padding_all(mut self, value: f32) -> Self {
            self.style.padding_all(value);
            self
        }

        pub fn padding_symmetric(mut self, vertical: f32, horizontal: f32) -> Self {
            self.style.padding_symmetric(vertical, horizontal);
            self
        }

        pub fn padding_top(mut self, value: f32) -> Self {
            self.style.padding_top(value);
            self
        }

        pub fn padding_bottom(mut self, value: f32) -> Self {
            self.style.padding_bottom(value);
            self
        }

        pub fn padding_left(mut self, value: f32) -> Self {
            self.style.padding_left(value);
            self
        }

        pub fn padding_right(mut self, value: f32) -> Self {
            self.style.padding_right(value);
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

        /// Make the [`Widget`] a fixed size.
        pub fn fixed(mut self, width: f32, height: f32) -> Self {
            self.style.fixed(width, height);
            self
        }
    };
}
