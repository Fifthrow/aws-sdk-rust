// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides information on boosting <code>STRING_LIST</code> type document attributes.</p><note>
/// <p>For <code>STRING</code> and <code>STRING_LIST</code> type document attributes to be used for boosting on the console and the API, they must be enabled for search using the <a href="https://docs.aws.amazon.com/amazonq/latest/api-reference/API_DocumentAttributeConfiguration.html">DocumentAttributeConfiguration</a> object of the <a href="https://docs.aws.amazon.com/amazonq/latest/api-reference/API_UpdateIndex.html">UpdateIndex</a> API. If you haven't enabled searching on these attributes, you can't boost attributes of these data types on either the console or the API.</p>
/// </note>
/// <p>For more information on how boosting document attributes work in Amazon Q, see <a href="https://docs.aws.amazon.com/amazonq/latest/business-use-dg/metadata-boosting.html">Boosting using document attributes</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StringListAttributeBoostingConfiguration {
    /// <p>Specifies how much a document attribute is boosted.</p>
    pub boosting_level: crate::types::DocumentAttributeBoostingLevel,
}
impl StringListAttributeBoostingConfiguration {
    /// <p>Specifies how much a document attribute is boosted.</p>
    pub fn boosting_level(&self) -> &crate::types::DocumentAttributeBoostingLevel {
        &self.boosting_level
    }
}
impl StringListAttributeBoostingConfiguration {
    /// Creates a new builder-style object to manufacture [`StringListAttributeBoostingConfiguration`](crate::types::StringListAttributeBoostingConfiguration).
    pub fn builder() -> crate::types::builders::StringListAttributeBoostingConfigurationBuilder {
        crate::types::builders::StringListAttributeBoostingConfigurationBuilder::default()
    }
}

/// A builder for [`StringListAttributeBoostingConfiguration`](crate::types::StringListAttributeBoostingConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StringListAttributeBoostingConfigurationBuilder {
    pub(crate) boosting_level: ::std::option::Option<crate::types::DocumentAttributeBoostingLevel>,
}
impl StringListAttributeBoostingConfigurationBuilder {
    /// <p>Specifies how much a document attribute is boosted.</p>
    /// This field is required.
    pub fn boosting_level(mut self, input: crate::types::DocumentAttributeBoostingLevel) -> Self {
        self.boosting_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies how much a document attribute is boosted.</p>
    pub fn set_boosting_level(mut self, input: ::std::option::Option<crate::types::DocumentAttributeBoostingLevel>) -> Self {
        self.boosting_level = input;
        self
    }
    /// <p>Specifies how much a document attribute is boosted.</p>
    pub fn get_boosting_level(&self) -> &::std::option::Option<crate::types::DocumentAttributeBoostingLevel> {
        &self.boosting_level
    }
    /// Consumes the builder and constructs a [`StringListAttributeBoostingConfiguration`](crate::types::StringListAttributeBoostingConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`boosting_level`](crate::types::builders::StringListAttributeBoostingConfigurationBuilder::boosting_level)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::StringListAttributeBoostingConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::StringListAttributeBoostingConfiguration {
            boosting_level: self.boosting_level.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "boosting_level",
                    "boosting_level was not specified but it is required when building StringListAttributeBoostingConfiguration",
                )
            })?,
        })
    }
}
