// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_maintenance_windows_for_target::_describe_maintenance_windows_for_target_output::DescribeMaintenanceWindowsForTargetOutputBuilder;

pub use crate::operation::describe_maintenance_windows_for_target::_describe_maintenance_windows_for_target_input::DescribeMaintenanceWindowsForTargetInputBuilder;

impl DescribeMaintenanceWindowsForTargetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_maintenance_windows_for_target();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeMaintenanceWindowsForTarget`.
///
/// <p>Retrieves information about the maintenance window targets or tasks that a managed node is associated with.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeMaintenanceWindowsForTargetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_maintenance_windows_for_target::builders::DescribeMaintenanceWindowsForTargetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetOutput,
        crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetError,
    > for DescribeMaintenanceWindowsForTargetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetOutput,
            crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeMaintenanceWindowsForTargetFluentBuilder {
    /// Creates a new `DescribeMaintenanceWindowsForTarget`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeMaintenanceWindowsForTarget as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_maintenance_windows_for_target::builders::DescribeMaintenanceWindowsForTargetInputBuilder {
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
        crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTarget::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTarget::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetOutput,
        crate::operation::describe_maintenance_windows_for_target::DescribeMaintenanceWindowsForTargetError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_maintenance_windows_for_target::paginator::DescribeMaintenanceWindowsForTargetPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_maintenance_windows_for_target::paginator::DescribeMaintenanceWindowsForTargetPaginator {
        crate::operation::describe_maintenance_windows_for_target::paginator::DescribeMaintenanceWindowsForTargetPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `Targets`.
    ///
    /// To override the contents of this collection use [`set_targets`](Self::set_targets).
    ///
    /// <p>The managed node ID or key-value pair to retrieve information about.</p>
    pub fn targets(mut self, input: crate::types::Target) -> Self {
        self.inner = self.inner.targets(input);
        self
    }
    /// <p>The managed node ID or key-value pair to retrieve information about.</p>
    pub fn set_targets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Target>>) -> Self {
        self.inner = self.inner.set_targets(input);
        self
    }
    /// <p>The managed node ID or key-value pair to retrieve information about.</p>
    pub fn get_targets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Target>> {
        self.inner.get_targets()
    }
    /// <p>The type of resource you want to retrieve information about. For example, <code>INSTANCE</code>.</p>
    pub fn resource_type(mut self, input: crate::types::MaintenanceWindowResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>The type of resource you want to retrieve information about. For example, <code>INSTANCE</code>.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<crate::types::MaintenanceWindowResourceType>) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>The type of resource you want to retrieve information about. For example, <code>INSTANCE</code>.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<crate::types::MaintenanceWindowResourceType> {
        self.inner.get_resource_type()
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
