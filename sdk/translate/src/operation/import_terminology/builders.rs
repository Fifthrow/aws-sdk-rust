// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_terminology::_import_terminology_output::ImportTerminologyOutputBuilder;

pub use crate::operation::import_terminology::_import_terminology_input::ImportTerminologyInputBuilder;

impl ImportTerminologyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::import_terminology::ImportTerminologyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_terminology::ImportTerminologyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.import_terminology();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ImportTerminology`.
///
/// <p>Creates or updates a custom terminology, depending on whether one already exists for the given terminology name. Importing a terminology with the same name as an existing one will merge the terminologies based on the chosen merge strategy. The only supported merge strategy is OVERWRITE, where the imported terminology overwrites the existing terminology of the same name.</p>
/// <p>If you import a terminology that overwrites an existing one, the new terminology takes up to 10 minutes to fully propagate. After that, translations have access to the new terminology.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportTerminologyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_terminology::builders::ImportTerminologyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::import_terminology::ImportTerminologyOutput,
        crate::operation::import_terminology::ImportTerminologyError,
    > for ImportTerminologyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::import_terminology::ImportTerminologyOutput,
            crate::operation::import_terminology::ImportTerminologyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ImportTerminologyFluentBuilder {
    /// Creates a new `ImportTerminology`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ImportTerminology as a reference.
    pub fn as_input(&self) -> &crate::operation::import_terminology::builders::ImportTerminologyInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::import_terminology::ImportTerminologyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_terminology::ImportTerminologyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::import_terminology::ImportTerminology::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::import_terminology::ImportTerminology::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::import_terminology::ImportTerminologyOutput,
        crate::operation::import_terminology::ImportTerminologyError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the custom terminology being imported.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the custom terminology being imported.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the custom terminology being imported.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The merge strategy of the custom terminology being imported. Currently, only the OVERWRITE merge strategy is supported. In this case, the imported terminology will overwrite an existing terminology of the same name.</p>
    pub fn merge_strategy(mut self, input: crate::types::MergeStrategy) -> Self {
        self.inner = self.inner.merge_strategy(input);
        self
    }
    /// <p>The merge strategy of the custom terminology being imported. Currently, only the OVERWRITE merge strategy is supported. In this case, the imported terminology will overwrite an existing terminology of the same name.</p>
    pub fn set_merge_strategy(mut self, input: ::std::option::Option<crate::types::MergeStrategy>) -> Self {
        self.inner = self.inner.set_merge_strategy(input);
        self
    }
    /// <p>The merge strategy of the custom terminology being imported. Currently, only the OVERWRITE merge strategy is supported. In this case, the imported terminology will overwrite an existing terminology of the same name.</p>
    pub fn get_merge_strategy(&self) -> &::std::option::Option<crate::types::MergeStrategy> {
        self.inner.get_merge_strategy()
    }
    /// <p>The description of the custom terminology being imported.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the custom terminology being imported.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the custom terminology being imported.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The terminology data for the custom terminology being imported.</p>
    pub fn terminology_data(mut self, input: crate::types::TerminologyData) -> Self {
        self.inner = self.inner.terminology_data(input);
        self
    }
    /// <p>The terminology data for the custom terminology being imported.</p>
    pub fn set_terminology_data(mut self, input: ::std::option::Option<crate::types::TerminologyData>) -> Self {
        self.inner = self.inner.set_terminology_data(input);
        self
    }
    /// <p>The terminology data for the custom terminology being imported.</p>
    pub fn get_terminology_data(&self) -> &::std::option::Option<crate::types::TerminologyData> {
        self.inner.get_terminology_data()
    }
    /// <p>The encryption key for the custom terminology being imported.</p>
    pub fn encryption_key(mut self, input: crate::types::EncryptionKey) -> Self {
        self.inner = self.inner.encryption_key(input);
        self
    }
    /// <p>The encryption key for the custom terminology being imported.</p>
    pub fn set_encryption_key(mut self, input: ::std::option::Option<crate::types::EncryptionKey>) -> Self {
        self.inner = self.inner.set_encryption_key(input);
        self
    }
    /// <p>The encryption key for the custom terminology being imported.</p>
    pub fn get_encryption_key(&self) -> &::std::option::Option<crate::types::EncryptionKey> {
        self.inner.get_encryption_key()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags to be associated with this resource. A tag is a key-value pair that adds metadata to a resource. Each tag key for the resource must be unique. For more information, see <a href="https://docs.aws.amazon.com/translate/latest/dg/tagging.html"> Tagging your resources</a>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Tags to be associated with this resource. A tag is a key-value pair that adds metadata to a resource. Each tag key for the resource must be unique. For more information, see <a href="https://docs.aws.amazon.com/translate/latest/dg/tagging.html"> Tagging your resources</a>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Tags to be associated with this resource. A tag is a key-value pair that adds metadata to a resource. Each tag key for the resource must be unique. For more information, see <a href="https://docs.aws.amazon.com/translate/latest/dg/tagging.html"> Tagging your resources</a>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
