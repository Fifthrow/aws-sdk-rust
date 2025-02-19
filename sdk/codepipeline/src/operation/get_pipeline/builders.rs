// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_pipeline::_get_pipeline_output::GetPipelineOutputBuilder;

pub use crate::operation::get_pipeline::_get_pipeline_input::GetPipelineInputBuilder;

impl GetPipelineInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_pipeline::GetPipelineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_pipeline::GetPipelineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_pipeline();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPipeline`.
///
/// <p>Returns the metadata, structure, stages, and actions of a pipeline. Can be used to return the entire structure of a pipeline in JSON format, which can then be modified and used to update the pipeline structure with <code>UpdatePipeline</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPipelineFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_pipeline::builders::GetPipelineInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_pipeline::GetPipelineOutput,
        crate::operation::get_pipeline::GetPipelineError,
    > for GetPipelineFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_pipeline::GetPipelineOutput,
            crate::operation::get_pipeline::GetPipelineError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPipelineFluentBuilder {
    /// Creates a new `GetPipeline`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPipeline as a reference.
    pub fn as_input(&self) -> &crate::operation::get_pipeline::builders::GetPipelineInputBuilder {
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
        crate::operation::get_pipeline::GetPipelineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_pipeline::GetPipelineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_pipeline::GetPipeline::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_pipeline::GetPipeline::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_pipeline::GetPipelineOutput,
        crate::operation::get_pipeline::GetPipelineError,
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
    /// <p>The name of the pipeline for which you want to get information. Pipeline names must be unique in an Amazon Web Services account.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the pipeline for which you want to get information. Pipeline names must be unique in an Amazon Web Services account.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the pipeline for which you want to get information. Pipeline names must be unique in an Amazon Web Services account.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The version number of the pipeline. If you do not specify a version, defaults to the current version.</p>
    pub fn version(mut self, input: i32) -> Self {
        self.inner = self.inner.version(input);
        self
    }
    /// <p>The version number of the pipeline. If you do not specify a version, defaults to the current version.</p>
    pub fn set_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_version(input);
        self
    }
    /// <p>The version number of the pipeline. If you do not specify a version, defaults to the current version.</p>
    pub fn get_version(&self) -> &::std::option::Option<i32> {
        self.inner.get_version()
    }
}
