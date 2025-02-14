//! Create embed fields.

use randy_model::channel::message::embed::EmbedField;

/// Create an embed field with a builder.
///
/// This can be passed into [`EmbedBuilder::field`].
///
/// Fields are not inlined by default. Use [`inline`] to inline a field.
///
/// [`EmbedBuilder::field`]: crate::builder::embed::EmbedBuilder::field
/// [`inline`]: Self::inline
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed field"]
pub struct EmbedFieldBuilder(EmbedField);

impl EmbedFieldBuilder {
    /// Create a new embed field builder.
    ///
    /// Refer to [`FIELD_NAME_LENGTH`] for the maximum number of UTF-16 code
    /// points that can be in a field name.
    ///
    /// Refer to [`FIELD_VALUE_LENGTH`] for the maximum number of UTF-16 code
    /// points that can be in a field value.
    ///
    /// [`FIELD_NAME_LENGTH`]: randy_validate::embed::FIELD_NAME_LENGTH
    /// [`FIELD_VALUE_LENGTH`]: randy_validate::embed::FIELD_VALUE_LENGTH
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self(EmbedField {
            inline: false,
            name: name.into(),
            value: value.into(),
        })
    }

    /// Build into an embed field.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used as part of an embed builder"]
    pub fn build(self) -> EmbedField {
        self.0
    }

    /// Inline the field.
    ///
    /// # Examples
    ///
    /// Create an inlined field:
    ///
    /// ```no_run
    /// use twilight_util::builder::embed::EmbedFieldBuilder;
    ///
    /// let field = EmbedFieldBuilder::new("twilight", "is cool")
    ///     .inline()
    ///     .build();
    /// ```
    pub const fn inline(mut self) -> Self {
        self.0.inline = true;

        self
    }
}

impl From<EmbedFieldBuilder> for EmbedField {
    /// Convert an embed field builder into an embed field.
    ///
    /// This is equivalent to calling [`EmbedFieldBuilder::build`].
    fn from(builder: EmbedFieldBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(EmbedFieldBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(EmbedField: From<EmbedFieldBuilder>);

    #[test]
    fn builder_inline() {
        let expected = EmbedField {
            inline: true,
            name: "name".to_owned(),
            value: "value".to_owned(),
        };
        let actual = EmbedFieldBuilder::new("name", "value").inline().build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn builder_no_inline() {
        let expected = EmbedField {
            inline: false,
            name: "name".to_owned(),
            value: "value".to_owned(),
        };
        let actual = EmbedFieldBuilder::new("name", "value").build();

        assert_eq!(actual, expected);
    }
}
