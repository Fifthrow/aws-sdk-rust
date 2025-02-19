// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_skills::_list_skills_output::ListSkillsOutputBuilder;

pub use crate::operation::list_skills::_list_skills_input::ListSkillsInputBuilder;

impl ListSkillsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_skills::ListSkillsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_skills::ListSkillsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_skills();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSkills`.
///
/// <p>Lists all enabled skills in a specific skill group.</p>
#[deprecated(note = "Alexa For Business is no longer supported")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSkillsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_skills::builders::ListSkillsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_skills::ListSkillsOutput,
        crate::operation::list_skills::ListSkillsError,
    > for ListSkillsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_skills::ListSkillsOutput,
            crate::operation::list_skills::ListSkillsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListSkillsFluentBuilder {
    /// Creates a new `ListSkills`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSkills as a reference.
    pub fn as_input(&self) -> &crate::operation::list_skills::builders::ListSkillsInputBuilder {
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
        crate::operation::list_skills::ListSkillsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_skills::ListSkillsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_skills::ListSkills::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_skills::ListSkills::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_skills::ListSkillsOutput,
        crate::operation::list_skills::ListSkillsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_skills::paginator::ListSkillsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_skills::paginator::ListSkillsPaginator {
        crate::operation::list_skills::paginator::ListSkillsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ARN of the skill group for which to list enabled skills.</p>
    pub fn skill_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.skill_group_arn(input.into());
        self
    }
    /// <p>The ARN of the skill group for which to list enabled skills.</p>
    pub fn set_skill_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_skill_group_arn(input);
        self
    }
    /// <p>The ARN of the skill group for which to list enabled skills.</p>
    pub fn get_skill_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_skill_group_arn()
    }
    /// <p>Whether the skill is enabled under the user's account.</p>
    pub fn enablement_type(mut self, input: crate::types::EnablementTypeFilter) -> Self {
        self.inner = self.inner.enablement_type(input);
        self
    }
    /// <p>Whether the skill is enabled under the user's account.</p>
    pub fn set_enablement_type(mut self, input: ::std::option::Option<crate::types::EnablementTypeFilter>) -> Self {
        self.inner = self.inner.set_enablement_type(input);
        self
    }
    /// <p>Whether the skill is enabled under the user's account.</p>
    pub fn get_enablement_type(&self) -> &::std::option::Option<crate::types::EnablementTypeFilter> {
        self.inner.get_enablement_type()
    }
    /// <p>Whether the skill is publicly available or is a private skill.</p>
    pub fn skill_type(mut self, input: crate::types::SkillTypeFilter) -> Self {
        self.inner = self.inner.skill_type(input);
        self
    }
    /// <p>Whether the skill is publicly available or is a private skill.</p>
    pub fn set_skill_type(mut self, input: ::std::option::Option<crate::types::SkillTypeFilter>) -> Self {
        self.inner = self.inner.set_skill_type(input);
        self
    }
    /// <p>Whether the skill is publicly available or is a private skill.</p>
    pub fn get_skill_type(&self) -> &::std::option::Option<crate::types::SkillTypeFilter> {
        self.inner.get_skill_type()
    }
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
