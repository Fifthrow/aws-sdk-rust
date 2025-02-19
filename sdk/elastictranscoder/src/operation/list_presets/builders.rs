// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_presets::_list_presets_output::ListPresetsOutputBuilder;

pub use crate::operation::list_presets::_list_presets_input::ListPresetsInputBuilder;

impl ListPresetsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_presets::ListPresetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_presets::ListPresetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_presets();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListPresets`.
///
/// <p>The ListPresets operation gets a list of the default presets included with Elastic Transcoder and the presets that you've added in an AWS region.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPresetsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_presets::builders::ListPresetsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_presets::ListPresetsOutput,
        crate::operation::list_presets::ListPresetsError,
    > for ListPresetsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_presets::ListPresetsOutput,
            crate::operation::list_presets::ListPresetsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListPresetsFluentBuilder {
    /// Creates a new `ListPresets`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListPresets as a reference.
    pub fn as_input(&self) -> &crate::operation::list_presets::builders::ListPresetsInputBuilder {
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
        crate::operation::list_presets::ListPresetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_presets::ListPresetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_presets::ListPresets::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_presets::ListPresets::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_presets::ListPresetsOutput,
        crate::operation::list_presets::ListPresetsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_presets::paginator::ListPresetsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_presets::paginator::ListPresetsPaginator {
        crate::operation::list_presets::paginator::ListPresetsPaginator::new(self.handle, self.inner)
    }
    /// <p>To list presets in chronological order by the date and time that they were created, enter <code>true</code>. To list presets in reverse chronological order, enter <code>false</code>.</p>
    pub fn ascending(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ascending(input.into());
        self
    }
    /// <p>To list presets in chronological order by the date and time that they were created, enter <code>true</code>. To list presets in reverse chronological order, enter <code>false</code>.</p>
    pub fn set_ascending(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ascending(input);
        self
    }
    /// <p>To list presets in chronological order by the date and time that they were created, enter <code>true</code>. To list presets in reverse chronological order, enter <code>false</code>.</p>
    pub fn get_ascending(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ascending()
    }
    /// <p>When Elastic Transcoder returns more than one page of results, use <code>pageToken</code> in subsequent <code>GET</code> requests to get each successive page of results.</p>
    pub fn page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.page_token(input.into());
        self
    }
    /// <p>When Elastic Transcoder returns more than one page of results, use <code>pageToken</code> in subsequent <code>GET</code> requests to get each successive page of results.</p>
    pub fn set_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_page_token(input);
        self
    }
    /// <p>When Elastic Transcoder returns more than one page of results, use <code>pageToken</code> in subsequent <code>GET</code> requests to get each successive page of results.</p>
    pub fn get_page_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_page_token()
    }
}
