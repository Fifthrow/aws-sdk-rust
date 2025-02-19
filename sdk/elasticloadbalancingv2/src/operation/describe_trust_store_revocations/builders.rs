// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_trust_store_revocations::_describe_trust_store_revocations_output::DescribeTrustStoreRevocationsOutputBuilder;

pub use crate::operation::describe_trust_store_revocations::_describe_trust_store_revocations_input::DescribeTrustStoreRevocationsInputBuilder;

impl DescribeTrustStoreRevocationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_trust_store_revocations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeTrustStoreRevocations`.
///
/// <p>Describes the revocation files in use by the specified trust store arn, or revocation ID.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeTrustStoreRevocationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_trust_store_revocations::builders::DescribeTrustStoreRevocationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsOutput,
        crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsError,
    > for DescribeTrustStoreRevocationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsOutput,
            crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeTrustStoreRevocationsFluentBuilder {
    /// Creates a new `DescribeTrustStoreRevocations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeTrustStoreRevocations as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_trust_store_revocations::builders::DescribeTrustStoreRevocationsInputBuilder {
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
        crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsOutput,
        crate::operation::describe_trust_store_revocations::DescribeTrustStoreRevocationsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_trust_store_revocations::paginator::DescribeTrustStoreRevocationsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_trust_store_revocations::paginator::DescribeTrustStoreRevocationsPaginator {
        crate::operation::describe_trust_store_revocations::paginator::DescribeTrustStoreRevocationsPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) of the trust store.</p>
    pub fn trust_store_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.trust_store_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the trust store.</p>
    pub fn set_trust_store_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_trust_store_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the trust store.</p>
    pub fn get_trust_store_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_trust_store_arn()
    }
    /// Appends an item to `RevocationIds`.
    ///
    /// To override the contents of this collection use [`set_revocation_ids`](Self::set_revocation_ids).
    ///
    /// <p>The revocation IDs of the revocation files you want to describe.</p>
    pub fn revocation_ids(mut self, input: i64) -> Self {
        self.inner = self.inner.revocation_ids(input);
        self
    }
    /// <p>The revocation IDs of the revocation files you want to describe.</p>
    pub fn set_revocation_ids(mut self, input: ::std::option::Option<::std::vec::Vec<i64>>) -> Self {
        self.inner = self.inner.set_revocation_ids(input);
        self
    }
    /// <p>The revocation IDs of the revocation files you want to describe.</p>
    pub fn get_revocation_ids(&self) -> &::std::option::Option<::std::vec::Vec<i64>> {
        self.inner.get_revocation_ids()
    }
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>The maximum number of results to return with this call.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The maximum number of results to return with this call.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The maximum number of results to return with this call.</p>
    pub fn get_page_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_page_size()
    }
}
