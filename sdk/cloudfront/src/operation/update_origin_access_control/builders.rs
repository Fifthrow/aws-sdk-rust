// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_origin_access_control::_update_origin_access_control_output::UpdateOriginAccessControlOutputBuilder;

pub use crate::operation::update_origin_access_control::_update_origin_access_control_input::UpdateOriginAccessControlInputBuilder;

impl UpdateOriginAccessControlInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_origin_access_control::UpdateOriginAccessControlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_origin_access_control::UpdateOriginAccessControlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_origin_access_control();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateOriginAccessControl`.
///
/// <p>Updates a CloudFront origin access control.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateOriginAccessControlFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_origin_access_control::builders::UpdateOriginAccessControlInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_origin_access_control::UpdateOriginAccessControlOutput,
        crate::operation::update_origin_access_control::UpdateOriginAccessControlError,
    > for UpdateOriginAccessControlFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_origin_access_control::UpdateOriginAccessControlOutput,
            crate::operation::update_origin_access_control::UpdateOriginAccessControlError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateOriginAccessControlFluentBuilder {
    /// Creates a new `UpdateOriginAccessControl`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateOriginAccessControl as a reference.
    pub fn as_input(&self) -> &crate::operation::update_origin_access_control::builders::UpdateOriginAccessControlInputBuilder {
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
        crate::operation::update_origin_access_control::UpdateOriginAccessControlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_origin_access_control::UpdateOriginAccessControlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_origin_access_control::UpdateOriginAccessControl::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_origin_access_control::UpdateOriginAccessControl::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_origin_access_control::UpdateOriginAccessControlOutput,
        crate::operation::update_origin_access_control::UpdateOriginAccessControlError,
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
    /// <p>An origin access control.</p>
    pub fn origin_access_control_config(mut self, input: crate::types::OriginAccessControlConfig) -> Self {
        self.inner = self.inner.origin_access_control_config(input);
        self
    }
    /// <p>An origin access control.</p>
    pub fn set_origin_access_control_config(mut self, input: ::std::option::Option<crate::types::OriginAccessControlConfig>) -> Self {
        self.inner = self.inner.set_origin_access_control_config(input);
        self
    }
    /// <p>An origin access control.</p>
    pub fn get_origin_access_control_config(&self) -> &::std::option::Option<crate::types::OriginAccessControlConfig> {
        self.inner.get_origin_access_control_config()
    }
    /// <p>The unique identifier of the origin access control that you are updating.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The unique identifier of the origin access control that you are updating.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The unique identifier of the origin access control that you are updating.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The current version (<code>ETag</code> value) of the origin access control that you are updating.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The current version (<code>ETag</code> value) of the origin access control that you are updating.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
    /// <p>The current version (<code>ETag</code> value) of the origin access control that you are updating.</p>
    pub fn get_if_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_if_match()
    }
}
