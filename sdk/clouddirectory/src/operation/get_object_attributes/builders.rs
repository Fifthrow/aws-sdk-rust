// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_object_attributes::_get_object_attributes_output::GetObjectAttributesOutputBuilder;

pub use crate::operation::get_object_attributes::_get_object_attributes_input::GetObjectAttributesInputBuilder;

impl GetObjectAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_object_attributes::GetObjectAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_object_attributes::GetObjectAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_object_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetObjectAttributes`.
///
/// <p>Retrieves attributes within a facet that are associated with an object.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetObjectAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_object_attributes::builders::GetObjectAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_object_attributes::GetObjectAttributesOutput,
        crate::operation::get_object_attributes::GetObjectAttributesError,
    > for GetObjectAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_object_attributes::GetObjectAttributesOutput,
            crate::operation::get_object_attributes::GetObjectAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetObjectAttributesFluentBuilder {
    /// Creates a new `GetObjectAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetObjectAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::get_object_attributes::builders::GetObjectAttributesInputBuilder {
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
        crate::operation::get_object_attributes::GetObjectAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_object_attributes::GetObjectAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_object_attributes::GetObjectAttributes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_object_attributes::GetObjectAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_object_attributes::GetObjectAttributesOutput,
        crate::operation::get_object_attributes::GetObjectAttributesError,
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
    /// <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where the object resides.</p>
    pub fn directory_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where the object resides.</p>
    pub fn set_directory_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where the object resides.</p>
    pub fn get_directory_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_arn()
    }
    /// <p>Reference that identifies the object whose attributes will be retrieved.</p>
    pub fn object_reference(mut self, input: crate::types::ObjectReference) -> Self {
        self.inner = self.inner.object_reference(input);
        self
    }
    /// <p>Reference that identifies the object whose attributes will be retrieved.</p>
    pub fn set_object_reference(mut self, input: ::std::option::Option<crate::types::ObjectReference>) -> Self {
        self.inner = self.inner.set_object_reference(input);
        self
    }
    /// <p>Reference that identifies the object whose attributes will be retrieved.</p>
    pub fn get_object_reference(&self) -> &::std::option::Option<crate::types::ObjectReference> {
        self.inner.get_object_reference()
    }
    /// <p>The consistency level at which to retrieve the attributes on an object.</p>
    pub fn consistency_level(mut self, input: crate::types::ConsistencyLevel) -> Self {
        self.inner = self.inner.consistency_level(input);
        self
    }
    /// <p>The consistency level at which to retrieve the attributes on an object.</p>
    pub fn set_consistency_level(mut self, input: ::std::option::Option<crate::types::ConsistencyLevel>) -> Self {
        self.inner = self.inner.set_consistency_level(input);
        self
    }
    /// <p>The consistency level at which to retrieve the attributes on an object.</p>
    pub fn get_consistency_level(&self) -> &::std::option::Option<crate::types::ConsistencyLevel> {
        self.inner.get_consistency_level()
    }
    /// <p>Identifier for the facet whose attributes will be retrieved. See <code>SchemaFacet</code> for details.</p>
    pub fn schema_facet(mut self, input: crate::types::SchemaFacet) -> Self {
        self.inner = self.inner.schema_facet(input);
        self
    }
    /// <p>Identifier for the facet whose attributes will be retrieved. See <code>SchemaFacet</code> for details.</p>
    pub fn set_schema_facet(mut self, input: ::std::option::Option<crate::types::SchemaFacet>) -> Self {
        self.inner = self.inner.set_schema_facet(input);
        self
    }
    /// <p>Identifier for the facet whose attributes will be retrieved. See <code>SchemaFacet</code> for details.</p>
    pub fn get_schema_facet(&self) -> &::std::option::Option<crate::types::SchemaFacet> {
        self.inner.get_schema_facet()
    }
    /// Appends an item to `AttributeNames`.
    ///
    /// To override the contents of this collection use [`set_attribute_names`](Self::set_attribute_names).
    ///
    /// <p>List of attribute names whose values will be retrieved.</p>
    pub fn attribute_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attribute_names(input.into());
        self
    }
    /// <p>List of attribute names whose values will be retrieved.</p>
    pub fn set_attribute_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_attribute_names(input);
        self
    }
    /// <p>List of attribute names whose values will be retrieved.</p>
    pub fn get_attribute_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_attribute_names()
    }
}
