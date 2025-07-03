/// Implement common styling attributes
#[macro_export]
macro_rules! impl_style {
    () => {
        /// Change the [`Color`] of a [`Widget`].
        pub fn color(mut self, color: impl $crate::IntoColor<$crate::Rgba>) -> Self {
            self.color = color.into_color();
            self
        }
    };
}

/// Implement common methods for widgets
#[macro_export]
macro_rules! impl_modifiers {
    () => {
        pub fn fill(mut self) -> Self {
            self.modifiers.intrinsic_size.width = agape_layout::BoxSizing::Flex(1);
            self.modifiers.intrinsic_size.height = agape_layout::BoxSizing::Flex(1);
            self
        }

        pub fn flex(mut self, factor: u8) -> Self {
            self.modifiers.intrinsic_size.width = agape_layout::BoxSizing::Flex(factor);
            self.modifiers.intrinsic_size.height = agape_layout::BoxSizing::Flex(factor);
            self
        }

        pub fn fit(mut self) -> Self {
            self.modifiers.intrinsic_size.width = agape_layout::BoxSizing::Shrink;
            self.modifiers.intrinsic_size.height = agape_layout::BoxSizing::Shrink;
            self
        }

        pub fn fill_width(mut self) -> Self {
            self.modifiers.intrinsic_size.width = agape_layout::BoxSizing::Flex(1);
            self
        }

        pub fn fill_height(mut self) -> Self {
            self.modifiers.intrinsic_size.height = agape_layout::BoxSizing::Flex(1);
            self
        }

        pub fn fixed_width(mut self, width: f32) -> Self {
            self.modifiers.intrinsic_size.width = agape_layout::BoxSizing::Fixed(width);
            self
        }

        pub fn fixed_height(mut self, height: f32) -> Self {
            self.modifiers.intrinsic_size.height = agape_layout::BoxSizing::Fixed(height);
            self
        }

        pub fn fit_width(mut self) -> Self {
            self.modifiers.intrinsic_size.width = agape_layout::BoxSizing::Shrink;
            self
        }

        pub fn fit_height(mut self) -> Self {
            self.modifiers.intrinsic_size.height = agape_layout::BoxSizing::Shrink;
            self
        }

        pub fn flex_width(mut self, factor: u8) -> Self {
            self.modifiers.intrinsic_size.height = agape_layout::BoxSizing::Flex(factor);
            self
        }

        pub fn flex_height(mut self, factor: u8) -> Self {
            self.modifiers.intrinsic_size.height = agape_layout::BoxSizing::Flex(factor);
            self
        }
    };
}

#[macro_export]
macro_rules! impl_layout {
    () => {
        pub fn fill(mut self) -> Self {
            self.layout.intrinsic_size.width = agape_layout::BoxSizing::Flex(1);
            self.layout.intrinsic_size.height = agape_layout::BoxSizing::Flex(1);
            self
        }

        pub fn flex(mut self, factor: u8) -> Self {
            self.layout.intrinsic_size.width = agape_layout::BoxSizing::Flex(factor);
            self.layout.intrinsic_size.height = agape_layout::BoxSizing::Flex(factor);
            self
        }

        pub fn fit(mut self) -> Self {
            self.layout.intrinsic_size.width = agape_layout::BoxSizing::Shrink;
            self.layout.intrinsic_size.height = agape_layout::BoxSizing::Shrink;
            self
        }

        pub fn fill_width(mut self) -> Self {
            self.layout.intrinsic_size.width = agape_layout::BoxSizing::Flex(1);
            self
        }

        pub fn fill_height(mut self) -> Self {
            self.layout.intrinsic_size.height = agape_layout::BoxSizing::Flex(1);
            self
        }

        pub fn fixed_width(mut self, width: f32) -> Self {
            self.layout.intrinsic_size.width = agape_layout::BoxSizing::Fixed(width);
            self
        }

        pub fn fixed_height(mut self, height: f32) -> Self {
            self.layout.intrinsic_size.height = agape_layout::BoxSizing::Fixed(height);
            self
        }

        pub fn fit_width(mut self) -> Self {
            self.layout.intrinsic_size.width = agape_layout::BoxSizing::Shrink;
            self
        }

        pub fn fit_height(mut self) -> Self {
            self.layout.intrinsic_size.height = agape_layout::BoxSizing::Shrink;
            self
        }

        pub fn flex_width(mut self, factor: u8) -> Self {
            self.layout.intrinsic_size.height = agape_layout::BoxSizing::Flex(factor);
            self
        }

        pub fn flex_height(mut self, factor: u8) -> Self {
            self.layout.intrinsic_size.height = agape_layout::BoxSizing::Flex(factor);
            self
        }
    };
}
