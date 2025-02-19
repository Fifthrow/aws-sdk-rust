// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_cluster_security_group::_create_cluster_security_group_output::CreateClusterSecurityGroupOutputBuilder;

pub use crate::operation::create_cluster_security_group::_create_cluster_security_group_input::CreateClusterSecurityGroupInputBuilder;

impl CreateClusterSecurityGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_cluster_security_group::CreateClusterSecurityGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_cluster_security_group::CreateClusterSecurityGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_cluster_security_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateClusterSecurityGroup`.
///
/// <p>Creates a new Amazon Redshift security group. You use security groups to control access to non-VPC clusters.</p>
/// <p>For information about managing security groups, go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-security-groups.html">Amazon Redshift Cluster Security Groups</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateClusterSecurityGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_cluster_security_group::builders::CreateClusterSecurityGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_cluster_security_group::CreateClusterSecurityGroupOutput,
        crate::operation::create_cluster_security_group::CreateClusterSecurityGroupError,
    > for CreateClusterSecurityGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_cluster_security_group::CreateClusterSecurityGroupOutput,
            crate::operation::create_cluster_security_group::CreateClusterSecurityGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateClusterSecurityGroupFluentBuilder {
    /// Creates a new `CreateClusterSecurityGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateClusterSecurityGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::create_cluster_security_group::builders::CreateClusterSecurityGroupInputBuilder {
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
        crate::operation::create_cluster_security_group::CreateClusterSecurityGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_cluster_security_group::CreateClusterSecurityGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_cluster_security_group::CreateClusterSecurityGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_cluster_security_group::CreateClusterSecurityGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_cluster_security_group::CreateClusterSecurityGroupOutput,
        crate::operation::create_cluster_security_group::CreateClusterSecurityGroupError,
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
    /// <p>The name for the security group. Amazon Redshift stores the value as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must contain no more than 255 alphanumeric characters or hyphens.</p></li>
    /// <li>
    /// <p>Must not be "Default".</p></li>
    /// <li>
    /// <p>Must be unique for all security groups that are created by your Amazon Web Services account.</p></li>
    /// </ul>
    /// <p>Example: <code>examplesecuritygroup</code></p>
    pub fn cluster_security_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_security_group_name(input.into());
        self
    }
    /// <p>The name for the security group. Amazon Redshift stores the value as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must contain no more than 255 alphanumeric characters or hyphens.</p></li>
    /// <li>
    /// <p>Must not be "Default".</p></li>
    /// <li>
    /// <p>Must be unique for all security groups that are created by your Amazon Web Services account.</p></li>
    /// </ul>
    /// <p>Example: <code>examplesecuritygroup</code></p>
    pub fn set_cluster_security_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_security_group_name(input);
        self
    }
    /// <p>The name for the security group. Amazon Redshift stores the value as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must contain no more than 255 alphanumeric characters or hyphens.</p></li>
    /// <li>
    /// <p>Must not be "Default".</p></li>
    /// <li>
    /// <p>Must be unique for all security groups that are created by your Amazon Web Services account.</p></li>
    /// </ul>
    /// <p>Example: <code>examplesecuritygroup</code></p>
    pub fn get_cluster_security_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_security_group_name()
    }
    /// <p>A description for the security group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the security group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the security group.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tag instances.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of tag instances.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of tag instances.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
