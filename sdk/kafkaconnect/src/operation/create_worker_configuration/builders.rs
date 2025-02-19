// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_worker_configuration::_create_worker_configuration_output::CreateWorkerConfigurationOutputBuilder;

pub use crate::operation::create_worker_configuration::_create_worker_configuration_input::CreateWorkerConfigurationInputBuilder;

impl CreateWorkerConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_worker_configuration::CreateWorkerConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_worker_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateWorkerConfiguration`.
///
/// <p>Creates a worker configuration using the specified properties.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateWorkerConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput,
        crate::operation::create_worker_configuration::CreateWorkerConfigurationError,
    > for CreateWorkerConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput,
            crate::operation::create_worker_configuration::CreateWorkerConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateWorkerConfigurationFluentBuilder {
    /// Creates a new `CreateWorkerConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateWorkerConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::create_worker_configuration::builders::CreateWorkerConfigurationInputBuilder {
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
        crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_worker_configuration::CreateWorkerConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_worker_configuration::CreateWorkerConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_worker_configuration::CreateWorkerConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_worker_configuration::CreateWorkerConfigurationOutput,
        crate::operation::create_worker_configuration::CreateWorkerConfigurationError,
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
    /// <p>A summary description of the worker configuration.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A summary description of the worker configuration.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A summary description of the worker configuration.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The name of the worker configuration.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the worker configuration.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the worker configuration.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>Base64 encoded contents of connect-distributed.properties file.</p>
    pub fn properties_file_content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.properties_file_content(input.into());
        self
    }
    /// <p>Base64 encoded contents of connect-distributed.properties file.</p>
    pub fn set_properties_file_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_properties_file_content(input);
        self
    }
    /// <p>Base64 encoded contents of connect-distributed.properties file.</p>
    pub fn get_properties_file_content(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_properties_file_content()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags you want to attach to the worker configuration.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags you want to attach to the worker configuration.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags you want to attach to the worker configuration.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
