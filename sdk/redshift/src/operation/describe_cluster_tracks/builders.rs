// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_cluster_tracks::_describe_cluster_tracks_output::DescribeClusterTracksOutputBuilder;

pub use crate::operation::describe_cluster_tracks::_describe_cluster_tracks_input::DescribeClusterTracksInputBuilder;

impl DescribeClusterTracksInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_cluster_tracks::DescribeClusterTracksOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_cluster_tracks::DescribeClusterTracksError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_cluster_tracks();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeClusterTracks`.
///
/// <p>Returns a list of all the available maintenance tracks.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeClusterTracksFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_cluster_tracks::builders::DescribeClusterTracksInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_cluster_tracks::DescribeClusterTracksOutput,
        crate::operation::describe_cluster_tracks::DescribeClusterTracksError,
    > for DescribeClusterTracksFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_cluster_tracks::DescribeClusterTracksOutput,
            crate::operation::describe_cluster_tracks::DescribeClusterTracksError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeClusterTracksFluentBuilder {
    /// Creates a new `DescribeClusterTracks`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeClusterTracks as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_cluster_tracks::builders::DescribeClusterTracksInputBuilder {
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
        crate::operation::describe_cluster_tracks::DescribeClusterTracksOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_cluster_tracks::DescribeClusterTracksError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_cluster_tracks::DescribeClusterTracks::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_cluster_tracks::DescribeClusterTracks::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_cluster_tracks::DescribeClusterTracksOutput,
        crate::operation::describe_cluster_tracks::DescribeClusterTracksError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_cluster_tracks::paginator::DescribeClusterTracksPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_cluster_tracks::paginator::DescribeClusterTracksPaginator {
        crate::operation::describe_cluster_tracks::paginator::DescribeClusterTracksPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the maintenance track.</p>
    pub fn maintenance_track_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.maintenance_track_name(input.into());
        self
    }
    /// <p>The name of the maintenance track.</p>
    pub fn set_maintenance_track_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_maintenance_track_name(input);
        self
    }
    /// <p>The name of the maintenance track.</p>
    pub fn get_maintenance_track_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_maintenance_track_name()
    }
    /// <p>An integer value for the maximum number of maintenance tracks to return.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>An integer value for the maximum number of maintenance tracks to return.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>An integer value for the maximum number of maintenance tracks to return.</p>
    pub fn get_max_records(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_records()
    }
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeClusterTracks</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Redshift returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeClusterTracks</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Redshift returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeClusterTracks</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Redshift returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
}
