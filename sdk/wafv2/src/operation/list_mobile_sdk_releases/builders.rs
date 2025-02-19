// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_mobile_sdk_releases::_list_mobile_sdk_releases_output::ListMobileSdkReleasesOutputBuilder;

pub use crate::operation::list_mobile_sdk_releases::_list_mobile_sdk_releases_input::ListMobileSdkReleasesInputBuilder;

impl ListMobileSdkReleasesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_mobile_sdk_releases();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListMobileSdkReleases`.
///
/// <p>Retrieves a list of the available releases for the mobile SDK and the specified device platform.</p>
/// <p>The mobile SDK is not generally available. Customers who have access to the mobile SDK can use it to establish and manage WAF tokens for use in HTTP(S) requests from a mobile device to WAF. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-application-integration.html">WAF client application integration</a> in the <i>WAF Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListMobileSdkReleasesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_mobile_sdk_releases::builders::ListMobileSdkReleasesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesOutput,
        crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesError,
    > for ListMobileSdkReleasesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesOutput,
            crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListMobileSdkReleasesFluentBuilder {
    /// Creates a new `ListMobileSdkReleases`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListMobileSdkReleases as a reference.
    pub fn as_input(&self) -> &crate::operation::list_mobile_sdk_releases::builders::ListMobileSdkReleasesInputBuilder {
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
        crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_mobile_sdk_releases::ListMobileSdkReleases::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_mobile_sdk_releases::ListMobileSdkReleases::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesOutput,
        crate::operation::list_mobile_sdk_releases::ListMobileSdkReleasesError,
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
    /// <p>The device platform to retrieve the list for.</p>
    pub fn platform(mut self, input: crate::types::Platform) -> Self {
        self.inner = self.inner.platform(input);
        self
    }
    /// <p>The device platform to retrieve the list for.</p>
    pub fn set_platform(mut self, input: ::std::option::Option<crate::types::Platform>) -> Self {
        self.inner = self.inner.set_platform(input);
        self
    }
    /// <p>The device platform to retrieve the list for.</p>
    pub fn get_platform(&self) -> &::std::option::Option<crate::types::Platform> {
        self.inner.get_platform()
    }
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_marker(input.into());
        self
    }
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_marker(input);
        self
    }
    /// <p>When you request a list of objects with a <code>Limit</code> setting, if the number of objects that are still available for retrieval exceeds the limit, WAF returns a <code>NextMarker</code> value in the response. To retrieve the next batch of objects, provide the marker from the prior call in your next request.</p>
    pub fn get_next_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_marker()
    }
    /// <p>The maximum number of objects that you want WAF to return for this request. If more objects are available, in the response, WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of objects that you want WAF to return for this request. If more objects are available, in the response, WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of objects that you want WAF to return for this request. If more objects are available, in the response, WAF provides a <code>NextMarker</code> value that you can use in a subsequent call to get the next batch of objects.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
}
