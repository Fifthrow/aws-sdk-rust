// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_effective_patches_for_patch_baseline::_describe_effective_patches_for_patch_baseline_output::DescribeEffectivePatchesForPatchBaselineOutputBuilder;

pub use crate::operation::describe_effective_patches_for_patch_baseline::_describe_effective_patches_for_patch_baseline_input::DescribeEffectivePatchesForPatchBaselineInputBuilder;

impl DescribeEffectivePatchesForPatchBaselineInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_effective_patches_for_patch_baseline();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeEffectivePatchesForPatchBaseline`.
///
/// <p>Retrieves the current effective patches (the patch and the approval state) for the specified patch baseline. Applies to patch baselines for Windows only.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeEffectivePatchesForPatchBaselineFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_effective_patches_for_patch_baseline::builders::DescribeEffectivePatchesForPatchBaselineInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineOutput,
        crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineError,
    > for DescribeEffectivePatchesForPatchBaselineFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineOutput,
            crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeEffectivePatchesForPatchBaselineFluentBuilder {
    /// Creates a new `DescribeEffectivePatchesForPatchBaseline`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeEffectivePatchesForPatchBaseline as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_effective_patches_for_patch_baseline::builders::DescribeEffectivePatchesForPatchBaselineInputBuilder {
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
        crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaseline::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaseline::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineOutput,
        crate::operation::describe_effective_patches_for_patch_baseline::DescribeEffectivePatchesForPatchBaselineError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_effective_patches_for_patch_baseline::paginator::DescribeEffectivePatchesForPatchBaselinePaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_effective_patches_for_patch_baseline::paginator::DescribeEffectivePatchesForPatchBaselinePaginator {
        crate::operation::describe_effective_patches_for_patch_baseline::paginator::DescribeEffectivePatchesForPatchBaselinePaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The ID of the patch baseline to retrieve the effective patches for.</p>
    pub fn baseline_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.baseline_id(input.into());
        self
    }
    /// <p>The ID of the patch baseline to retrieve the effective patches for.</p>
    pub fn set_baseline_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_baseline_id(input);
        self
    }
    /// <p>The ID of the patch baseline to retrieve the effective patches for.</p>
    pub fn get_baseline_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_baseline_id()
    }
    /// <p>The maximum number of patches to return (per page).</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of patches to return (per page).</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of patches to return (per page).</p>
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
