// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_model_card_versions::_list_model_card_versions_output::ListModelCardVersionsOutputBuilder;

pub use crate::operation::list_model_card_versions::_list_model_card_versions_input::ListModelCardVersionsInputBuilder;

impl ListModelCardVersionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_model_card_versions::ListModelCardVersionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_model_card_versions::ListModelCardVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_model_card_versions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListModelCardVersions`.
///
/// <p>List existing versions of an Amazon SageMaker Model Card.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListModelCardVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_model_card_versions::builders::ListModelCardVersionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_model_card_versions::ListModelCardVersionsOutput,
        crate::operation::list_model_card_versions::ListModelCardVersionsError,
    > for ListModelCardVersionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_model_card_versions::ListModelCardVersionsOutput,
            crate::operation::list_model_card_versions::ListModelCardVersionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListModelCardVersionsFluentBuilder {
    /// Creates a new `ListModelCardVersions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListModelCardVersions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_model_card_versions::builders::ListModelCardVersionsInputBuilder {
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
        crate::operation::list_model_card_versions::ListModelCardVersionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_model_card_versions::ListModelCardVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_model_card_versions::ListModelCardVersions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_model_card_versions::ListModelCardVersions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_model_card_versions::ListModelCardVersionsOutput,
        crate::operation::list_model_card_versions::ListModelCardVersionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_model_card_versions::paginator::ListModelCardVersionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_model_card_versions::paginator::ListModelCardVersionsPaginator {
        crate::operation::list_model_card_versions::paginator::ListModelCardVersionsPaginator::new(self.handle, self.inner)
    }
    /// <p>Only list model card versions that were created after the time specified.</p>
    pub fn creation_time_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_after(input);
        self
    }
    /// <p>Only list model card versions that were created after the time specified.</p>
    pub fn set_creation_time_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_creation_time_after(input);
        self
    }
    /// <p>Only list model card versions that were created after the time specified.</p>
    pub fn get_creation_time_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_creation_time_after()
    }
    /// <p>Only list model card versions that were created before the time specified.</p>
    pub fn creation_time_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.creation_time_before(input);
        self
    }
    /// <p>Only list model card versions that were created before the time specified.</p>
    pub fn set_creation_time_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_creation_time_before(input);
        self
    }
    /// <p>Only list model card versions that were created before the time specified.</p>
    pub fn get_creation_time_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_creation_time_before()
    }
    /// <p>The maximum number of model card versions to list.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of model card versions to list.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of model card versions to list.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>List model card versions for the model card with the specified name or Amazon Resource Name (ARN).</p>
    pub fn model_card_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_card_name(input.into());
        self
    }
    /// <p>List model card versions for the model card with the specified name or Amazon Resource Name (ARN).</p>
    pub fn set_model_card_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_card_name(input);
        self
    }
    /// <p>List model card versions for the model card with the specified name or Amazon Resource Name (ARN).</p>
    pub fn get_model_card_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_model_card_name()
    }
    /// <p>Only list model card versions with the specified approval status.</p>
    pub fn model_card_status(mut self, input: crate::types::ModelCardStatus) -> Self {
        self.inner = self.inner.model_card_status(input);
        self
    }
    /// <p>Only list model card versions with the specified approval status.</p>
    pub fn set_model_card_status(mut self, input: ::std::option::Option<crate::types::ModelCardStatus>) -> Self {
        self.inner = self.inner.set_model_card_status(input);
        self
    }
    /// <p>Only list model card versions with the specified approval status.</p>
    pub fn get_model_card_status(&self) -> &::std::option::Option<crate::types::ModelCardStatus> {
        self.inner.get_model_card_status()
    }
    /// <p>If the response to a previous <code>ListModelCardVersions</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model card versions, use the token in the next request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the response to a previous <code>ListModelCardVersions</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model card versions, use the token in the next request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the response to a previous <code>ListModelCardVersions</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of model card versions, use the token in the next request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>Sort listed model card versions by version. Sorts by version by default.</p>
    pub fn sort_by(mut self, input: crate::types::ModelCardVersionSortBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>Sort listed model card versions by version. Sorts by version by default.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::ModelCardVersionSortBy>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>Sort listed model card versions by version. Sorts by version by default.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::ModelCardVersionSortBy> {
        self.inner.get_sort_by()
    }
    /// <p>Sort model card versions by ascending or descending order.</p>
    pub fn sort_order(mut self, input: crate::types::ModelCardSortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>Sort model card versions by ascending or descending order.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::ModelCardSortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>Sort model card versions by ascending or descending order.</p>
    pub fn get_sort_order(&self) -> &::std::option::Option<crate::types::ModelCardSortOrder> {
        self.inner.get_sort_order()
    }
}
