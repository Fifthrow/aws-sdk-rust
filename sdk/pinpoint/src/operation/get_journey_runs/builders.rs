// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_journey_runs::_get_journey_runs_output::GetJourneyRunsOutputBuilder;

pub use crate::operation::get_journey_runs::_get_journey_runs_input::GetJourneyRunsInputBuilder;

impl GetJourneyRunsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_journey_runs::GetJourneyRunsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_journey_runs::GetJourneyRunsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_journey_runs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetJourneyRuns`.
///
/// <p>Provides information about the runs of a journey.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetJourneyRunsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_journey_runs::builders::GetJourneyRunsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_journey_runs::GetJourneyRunsOutput,
        crate::operation::get_journey_runs::GetJourneyRunsError,
    > for GetJourneyRunsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_journey_runs::GetJourneyRunsOutput,
            crate::operation::get_journey_runs::GetJourneyRunsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetJourneyRunsFluentBuilder {
    /// Creates a new `GetJourneyRuns`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetJourneyRuns as a reference.
    pub fn as_input(&self) -> &crate::operation::get_journey_runs::builders::GetJourneyRunsInputBuilder {
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
        crate::operation::get_journey_runs::GetJourneyRunsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_journey_runs::GetJourneyRunsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_journey_runs::GetJourneyRuns::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_journey_runs::GetJourneyRuns::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_journey_runs::GetJourneyRunsOutput,
        crate::operation::get_journey_runs::GetJourneyRunsError,
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
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_id()
    }
    /// <p>The unique identifier for the journey.</p>
    pub fn journey_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.journey_id(input.into());
        self
    }
    /// <p>The unique identifier for the journey.</p>
    pub fn set_journey_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_journey_id(input);
        self
    }
    /// <p>The unique identifier for the journey.</p>
    pub fn get_journey_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_journey_id()
    }
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    pub fn page_size(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.page_size(input.into());
        self
    }
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    pub fn get_page_size(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_page_size()
    }
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    pub fn token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.token(input.into());
        self
    }
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    pub fn set_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_token(input);
        self
    }
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    pub fn get_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_token()
    }
}
