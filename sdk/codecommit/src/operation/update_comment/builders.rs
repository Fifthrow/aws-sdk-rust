// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_comment::_update_comment_output::UpdateCommentOutputBuilder;

pub use crate::operation::update_comment::_update_comment_input::UpdateCommentInputBuilder;

impl UpdateCommentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_comment::UpdateCommentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_comment::UpdateCommentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_comment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateComment`.
///
/// <p>Replaces the contents of a comment.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateCommentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_comment::builders::UpdateCommentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_comment::UpdateCommentOutput,
        crate::operation::update_comment::UpdateCommentError,
    > for UpdateCommentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_comment::UpdateCommentOutput,
            crate::operation::update_comment::UpdateCommentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateCommentFluentBuilder {
    /// Creates a new `UpdateComment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateComment as a reference.
    pub fn as_input(&self) -> &crate::operation::update_comment::builders::UpdateCommentInputBuilder {
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
        crate::operation::update_comment::UpdateCommentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_comment::UpdateCommentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_comment::UpdateComment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_comment::UpdateComment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_comment::UpdateCommentOutput,
        crate::operation::update_comment::UpdateCommentError,
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
    /// <p>The system-generated ID of the comment you want to update. To get this ID, use <code>GetCommentsForComparedCommit</code> or <code>GetCommentsForPullRequest</code>.</p>
    pub fn comment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.comment_id(input.into());
        self
    }
    /// <p>The system-generated ID of the comment you want to update. To get this ID, use <code>GetCommentsForComparedCommit</code> or <code>GetCommentsForPullRequest</code>.</p>
    pub fn set_comment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_comment_id(input);
        self
    }
    /// <p>The system-generated ID of the comment you want to update. To get this ID, use <code>GetCommentsForComparedCommit</code> or <code>GetCommentsForPullRequest</code>.</p>
    pub fn get_comment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_comment_id()
    }
    /// <p>The updated content to replace the existing content of the comment.</p>
    pub fn content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.content(input.into());
        self
    }
    /// <p>The updated content to replace the existing content of the comment.</p>
    pub fn set_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_content(input);
        self
    }
    /// <p>The updated content to replace the existing content of the comment.</p>
    pub fn get_content(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_content()
    }
}
