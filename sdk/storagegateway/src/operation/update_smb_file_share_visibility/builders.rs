// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_smb_file_share_visibility::_update_smb_file_share_visibility_output::UpdateSmbFileShareVisibilityOutputBuilder;

pub use crate::operation::update_smb_file_share_visibility::_update_smb_file_share_visibility_input::UpdateSmbFileShareVisibilityInputBuilder;

impl UpdateSmbFileShareVisibilityInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_smb_file_share_visibility::UpdateSmbFileShareVisibilityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_smb_file_share_visibility::UpdateSMBFileShareVisibilityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_smb_file_share_visibility();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateSMBFileShareVisibility`.
///
/// <p>Controls whether the shares on an S3 File Gateway are visible in a net view or browse list. The operation is only supported for S3 File Gateways.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSMBFileShareVisibilityFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_smb_file_share_visibility::builders::UpdateSmbFileShareVisibilityInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_smb_file_share_visibility::UpdateSmbFileShareVisibilityOutput,
        crate::operation::update_smb_file_share_visibility::UpdateSMBFileShareVisibilityError,
    > for UpdateSMBFileShareVisibilityFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_smb_file_share_visibility::UpdateSmbFileShareVisibilityOutput,
            crate::operation::update_smb_file_share_visibility::UpdateSMBFileShareVisibilityError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateSMBFileShareVisibilityFluentBuilder {
    /// Creates a new `UpdateSMBFileShareVisibility`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateSMBFileShareVisibility as a reference.
    pub fn as_input(&self) -> &crate::operation::update_smb_file_share_visibility::builders::UpdateSmbFileShareVisibilityInputBuilder {
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
        crate::operation::update_smb_file_share_visibility::UpdateSmbFileShareVisibilityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_smb_file_share_visibility::UpdateSMBFileShareVisibilityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_smb_file_share_visibility::UpdateSMBFileShareVisibility::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_smb_file_share_visibility::UpdateSMBFileShareVisibility::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_smb_file_share_visibility::UpdateSmbFileShareVisibilityOutput,
        crate::operation::update_smb_file_share_visibility::UpdateSMBFileShareVisibilityError,
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
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gateway_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn set_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn get_gateway_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_gateway_arn()
    }
    /// <p>The shares on this gateway appear when listing shares.</p>
    pub fn file_shares_visible(mut self, input: bool) -> Self {
        self.inner = self.inner.file_shares_visible(input);
        self
    }
    /// <p>The shares on this gateway appear when listing shares.</p>
    pub fn set_file_shares_visible(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_file_shares_visible(input);
        self
    }
    /// <p>The shares on this gateway appear when listing shares.</p>
    pub fn get_file_shares_visible(&self) -> &::std::option::Option<bool> {
        self.inner.get_file_shares_visible()
    }
}
