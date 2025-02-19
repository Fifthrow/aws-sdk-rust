// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_permissions::_update_permissions_output::UpdatePermissionsOutputBuilder;

pub use crate::operation::update_permissions::_update_permissions_input::UpdatePermissionsInputBuilder;

impl UpdatePermissionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_permissions::UpdatePermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_permissions::UpdatePermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_permissions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdatePermissions`.
///
/// <p>Updates which users in a workspace have the Grafana <code>Admin</code> or <code>Editor</code> roles.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePermissionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_permissions::builders::UpdatePermissionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_permissions::UpdatePermissionsOutput,
        crate::operation::update_permissions::UpdatePermissionsError,
    > for UpdatePermissionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_permissions::UpdatePermissionsOutput,
            crate::operation::update_permissions::UpdatePermissionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdatePermissionsFluentBuilder {
    /// Creates a new `UpdatePermissions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdatePermissions as a reference.
    pub fn as_input(&self) -> &crate::operation::update_permissions::builders::UpdatePermissionsInputBuilder {
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
        crate::operation::update_permissions::UpdatePermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_permissions::UpdatePermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_permissions::UpdatePermissions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_permissions::UpdatePermissions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_permissions::UpdatePermissionsOutput,
        crate::operation::update_permissions::UpdatePermissionsError,
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
    /// Appends an item to `updateInstructionBatch`.
    ///
    /// To override the contents of this collection use [`set_update_instruction_batch`](Self::set_update_instruction_batch).
    ///
    /// <p>An array of structures that contain the permission updates to make.</p>
    pub fn update_instruction_batch(mut self, input: crate::types::UpdateInstruction) -> Self {
        self.inner = self.inner.update_instruction_batch(input);
        self
    }
    /// <p>An array of structures that contain the permission updates to make.</p>
    pub fn set_update_instruction_batch(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UpdateInstruction>>) -> Self {
        self.inner = self.inner.set_update_instruction_batch(input);
        self
    }
    /// <p>An array of structures that contain the permission updates to make.</p>
    pub fn get_update_instruction_batch(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UpdateInstruction>> {
        self.inner.get_update_instruction_batch()
    }
    /// <p>The ID of the workspace to update.</p>
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workspace_id(input.into());
        self
    }
    /// <p>The ID of the workspace to update.</p>
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workspace_id(input);
        self
    }
    /// <p>The ID of the workspace to update.</p>
    pub fn get_workspace_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workspace_id()
    }
}
