// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_work_group::_delete_work_group_output::DeleteWorkGroupOutputBuilder;

pub use crate::operation::delete_work_group::_delete_work_group_input::DeleteWorkGroupInputBuilder;

impl DeleteWorkGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_work_group::DeleteWorkGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_work_group::DeleteWorkGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_work_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteWorkGroup`.
///
/// <p>Deletes the workgroup with the specified name. The primary workgroup cannot be deleted.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteWorkGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_work_group::builders::DeleteWorkGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_work_group::DeleteWorkGroupOutput,
        crate::operation::delete_work_group::DeleteWorkGroupError,
    > for DeleteWorkGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_work_group::DeleteWorkGroupOutput,
            crate::operation::delete_work_group::DeleteWorkGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteWorkGroupFluentBuilder {
    /// Creates a new `DeleteWorkGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteWorkGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_work_group::builders::DeleteWorkGroupInputBuilder {
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
        crate::operation::delete_work_group::DeleteWorkGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_work_group::DeleteWorkGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_work_group::DeleteWorkGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_work_group::DeleteWorkGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_work_group::DeleteWorkGroupOutput,
        crate::operation::delete_work_group::DeleteWorkGroupError,
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
    /// <p>The unique name of the workgroup to delete.</p>
    pub fn work_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.work_group(input.into());
        self
    }
    /// <p>The unique name of the workgroup to delete.</p>
    pub fn set_work_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_work_group(input);
        self
    }
    /// <p>The unique name of the workgroup to delete.</p>
    pub fn get_work_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_work_group()
    }
    /// <p>The option to delete the workgroup and its contents even if the workgroup contains any named queries, query executions, or notebooks.</p>
    pub fn recursive_delete_option(mut self, input: bool) -> Self {
        self.inner = self.inner.recursive_delete_option(input);
        self
    }
    /// <p>The option to delete the workgroup and its contents even if the workgroup contains any named queries, query executions, or notebooks.</p>
    pub fn set_recursive_delete_option(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_recursive_delete_option(input);
        self
    }
    /// <p>The option to delete the workgroup and its contents even if the workgroup contains any named queries, query executions, or notebooks.</p>
    pub fn get_recursive_delete_option(&self) -> &::std::option::Option<bool> {
        self.inner.get_recursive_delete_option()
    }
}
