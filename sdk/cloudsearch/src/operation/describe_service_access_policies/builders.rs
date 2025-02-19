// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_service_access_policies::_describe_service_access_policies_output::DescribeServiceAccessPoliciesOutputBuilder;

pub use crate::operation::describe_service_access_policies::_describe_service_access_policies_input::DescribeServiceAccessPoliciesInputBuilder;

impl DescribeServiceAccessPoliciesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_service_access_policies();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeServiceAccessPolicies`.
///
/// <p>Gets information about the access policies that control access to the domain's document and search endpoints. By default, shows the configuration with any pending changes. Set the <code>Deployed</code> option to <code>true</code> to show the active configuration and exclude pending changes. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-access.html" target="_blank">Configuring Access for a Search Domain</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeServiceAccessPoliciesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_service_access_policies::builders::DescribeServiceAccessPoliciesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesOutput,
        crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesError,
    > for DescribeServiceAccessPoliciesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesOutput,
            crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeServiceAccessPoliciesFluentBuilder {
    /// Creates a new `DescribeServiceAccessPolicies`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeServiceAccessPolicies as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_service_access_policies::builders::DescribeServiceAccessPoliciesInputBuilder {
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
        crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_service_access_policies::DescribeServiceAccessPolicies::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_service_access_policies::DescribeServiceAccessPolicies::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesOutput,
        crate::operation::describe_service_access_policies::DescribeServiceAccessPoliciesError,
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
    /// <p>The name of the domain you want to describe.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain you want to describe.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the domain you want to describe.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub fn deployed(mut self, input: bool) -> Self {
        self.inner = self.inner.deployed(input);
        self
    }
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub fn set_deployed(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deployed(input);
        self
    }
    /// <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
    pub fn get_deployed(&self) -> &::std::option::Option<bool> {
        self.inner.get_deployed()
    }
}
