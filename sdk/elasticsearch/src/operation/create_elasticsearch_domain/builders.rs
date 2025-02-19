// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_elasticsearch_domain::_create_elasticsearch_domain_output::CreateElasticsearchDomainOutputBuilder;

pub use crate::operation::create_elasticsearch_domain::_create_elasticsearch_domain_input::CreateElasticsearchDomainInputBuilder;

impl CreateElasticsearchDomainInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_elasticsearch_domain();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateElasticsearchDomain`.
///
/// <p>Creates a new Elasticsearch domain. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateElasticsearchDomainFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_elasticsearch_domain::builders::CreateElasticsearchDomainInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainOutput,
        crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainError,
    > for CreateElasticsearchDomainFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainOutput,
            crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateElasticsearchDomainFluentBuilder {
    /// Creates a new `CreateElasticsearchDomain`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateElasticsearchDomain as a reference.
    pub fn as_input(&self) -> &crate::operation::create_elasticsearch_domain::builders::CreateElasticsearchDomainInputBuilder {
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
        crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_elasticsearch_domain::CreateElasticsearchDomain::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_elasticsearch_domain::CreateElasticsearchDomain::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainOutput,
        crate::operation::create_elasticsearch_domain::CreateElasticsearchDomainError,
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
    /// <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the Elasticsearch domain that you are creating. Domain names are unique across the domains owned by an account within an AWS region. Domain names must start with a lowercase letter and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    pub fn elasticsearch_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.elasticsearch_version(input.into());
        self
    }
    /// <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    pub fn set_elasticsearch_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_elasticsearch_version(input);
        self
    }
    /// <p>String of format X.Y to specify version for the Elasticsearch domain eg. "1.5" or "2.3". For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomains" target="_blank">Creating Elasticsearch Domains</a> in the <i>Amazon Elasticsearch Service Developer Guide</i>.</p>
    pub fn get_elasticsearch_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_elasticsearch_version()
    }
    /// <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster.</p>
    pub fn elasticsearch_cluster_config(mut self, input: crate::types::ElasticsearchClusterConfig) -> Self {
        self.inner = self.inner.elasticsearch_cluster_config(input);
        self
    }
    /// <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster.</p>
    pub fn set_elasticsearch_cluster_config(mut self, input: ::std::option::Option<crate::types::ElasticsearchClusterConfig>) -> Self {
        self.inner = self.inner.set_elasticsearch_cluster_config(input);
        self
    }
    /// <p>Configuration options for an Elasticsearch domain. Specifies the instance type and number of instances in the domain cluster.</p>
    pub fn get_elasticsearch_cluster_config(&self) -> &::std::option::Option<crate::types::ElasticsearchClusterConfig> {
        self.inner.get_elasticsearch_cluster_config()
    }
    /// <p>Options to enable, disable and specify the type and size of EBS storage volumes.</p>
    pub fn ebs_options(mut self, input: crate::types::EbsOptions) -> Self {
        self.inner = self.inner.ebs_options(input);
        self
    }
    /// <p>Options to enable, disable and specify the type and size of EBS storage volumes.</p>
    pub fn set_ebs_options(mut self, input: ::std::option::Option<crate::types::EbsOptions>) -> Self {
        self.inner = self.inner.set_ebs_options(input);
        self
    }
    /// <p>Options to enable, disable and specify the type and size of EBS storage volumes.</p>
    pub fn get_ebs_options(&self) -> &::std::option::Option<crate::types::EbsOptions> {
        self.inner.get_ebs_options()
    }
    /// <p>IAM access policy as a JSON-formatted string.</p>
    pub fn access_policies(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.access_policies(input.into());
        self
    }
    /// <p>IAM access policy as a JSON-formatted string.</p>
    pub fn set_access_policies(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_access_policies(input);
        self
    }
    /// <p>IAM access policy as a JSON-formatted string.</p>
    pub fn get_access_policies(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_access_policies()
    }
    /// <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours.</p>
    pub fn snapshot_options(mut self, input: crate::types::SnapshotOptions) -> Self {
        self.inner = self.inner.snapshot_options(input);
        self
    }
    /// <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours.</p>
    pub fn set_snapshot_options(mut self, input: ::std::option::Option<crate::types::SnapshotOptions>) -> Self {
        self.inner = self.inner.set_snapshot_options(input);
        self
    }
    /// <p>Option to set time, in UTC format, of the daily automated snapshot. Default value is 0 hours.</p>
    pub fn get_snapshot_options(&self) -> &::std::option::Option<crate::types::SnapshotOptions> {
        self.inner.get_snapshot_options()
    }
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    pub fn vpc_options(mut self, input: crate::types::VpcOptions) -> Self {
        self.inner = self.inner.vpc_options(input);
        self
    }
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    pub fn set_vpc_options(mut self, input: ::std::option::Option<crate::types::VpcOptions>) -> Self {
        self.inner = self.inner.set_vpc_options(input);
        self
    }
    /// <p>Options to specify the subnets and security groups for VPC endpoint. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-creating-vpc" target="_blank">Creating a VPC</a> in <i>VPC Endpoints for Amazon Elasticsearch Service Domains</i></p>
    pub fn get_vpc_options(&self) -> &::std::option::Option<crate::types::VpcOptions> {
        self.inner.get_vpc_options()
    }
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    pub fn cognito_options(mut self, input: crate::types::CognitoOptions) -> Self {
        self.inner = self.inner.cognito_options(input);
        self
    }
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    pub fn set_cognito_options(mut self, input: ::std::option::Option<crate::types::CognitoOptions>) -> Self {
        self.inner = self.inner.set_cognito_options(input);
        self
    }
    /// <p>Options to specify the Cognito user and identity pools for Kibana authentication. For more information, see <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-cognito-auth.html" target="_blank">Amazon Cognito Authentication for Kibana</a>.</p>
    pub fn get_cognito_options(&self) -> &::std::option::Option<crate::types::CognitoOptions> {
        self.inner.get_cognito_options()
    }
    /// <p>Specifies the Encryption At Rest Options.</p>
    pub fn encryption_at_rest_options(mut self, input: crate::types::EncryptionAtRestOptions) -> Self {
        self.inner = self.inner.encryption_at_rest_options(input);
        self
    }
    /// <p>Specifies the Encryption At Rest Options.</p>
    pub fn set_encryption_at_rest_options(mut self, input: ::std::option::Option<crate::types::EncryptionAtRestOptions>) -> Self {
        self.inner = self.inner.set_encryption_at_rest_options(input);
        self
    }
    /// <p>Specifies the Encryption At Rest Options.</p>
    pub fn get_encryption_at_rest_options(&self) -> &::std::option::Option<crate::types::EncryptionAtRestOptions> {
        self.inner.get_encryption_at_rest_options()
    }
    /// <p>Specifies the NodeToNodeEncryptionOptions.</p>
    pub fn node_to_node_encryption_options(mut self, input: crate::types::NodeToNodeEncryptionOptions) -> Self {
        self.inner = self.inner.node_to_node_encryption_options(input);
        self
    }
    /// <p>Specifies the NodeToNodeEncryptionOptions.</p>
    pub fn set_node_to_node_encryption_options(mut self, input: ::std::option::Option<crate::types::NodeToNodeEncryptionOptions>) -> Self {
        self.inner = self.inner.set_node_to_node_encryption_options(input);
        self
    }
    /// <p>Specifies the NodeToNodeEncryptionOptions.</p>
    pub fn get_node_to_node_encryption_options(&self) -> &::std::option::Option<crate::types::NodeToNodeEncryptionOptions> {
        self.inner.get_node_to_node_encryption_options()
    }
    /// Adds a key-value pair to `AdvancedOptions`.
    ///
    /// To override the contents of this collection use [`set_advanced_options`](Self::set_advanced_options).
    ///
    /// <p>Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    pub fn advanced_options(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.advanced_options(k.into(), v.into());
        self
    }
    /// <p>Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    pub fn set_advanced_options(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_advanced_options(input);
        self
    }
    /// <p>Option to allow references to indices in an HTTP request body. Must be <code>false</code> when configuring access to individual sub-resources. By default, the value is <code>true</code>. See <a href="http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-createupdatedomains.html#es-createdomain-configure-advanced-options" target="_blank">Configuration Advanced Options</a> for more information.</p>
    pub fn get_advanced_options(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_advanced_options()
    }
    /// Adds a key-value pair to `LogPublishingOptions`.
    ///
    /// To override the contents of this collection use [`set_log_publishing_options`](Self::set_log_publishing_options).
    ///
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    pub fn log_publishing_options(mut self, k: crate::types::LogType, v: crate::types::LogPublishingOption) -> Self {
        self.inner = self.inner.log_publishing_options(k, v);
        self
    }
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    pub fn set_log_publishing_options(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<crate::types::LogType, crate::types::LogPublishingOption>>,
    ) -> Self {
        self.inner = self.inner.set_log_publishing_options(input);
        self
    }
    /// <p>Map of <code>LogType</code> and <code>LogPublishingOption</code>, each containing options to publish a given type of Elasticsearch log.</p>
    pub fn get_log_publishing_options(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<crate::types::LogType, crate::types::LogPublishingOption>> {
        self.inner.get_log_publishing_options()
    }
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    pub fn domain_endpoint_options(mut self, input: crate::types::DomainEndpointOptions) -> Self {
        self.inner = self.inner.domain_endpoint_options(input);
        self
    }
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    pub fn set_domain_endpoint_options(mut self, input: ::std::option::Option<crate::types::DomainEndpointOptions>) -> Self {
        self.inner = self.inner.set_domain_endpoint_options(input);
        self
    }
    /// <p>Options to specify configuration that will be applied to the domain endpoint.</p>
    pub fn get_domain_endpoint_options(&self) -> &::std::option::Option<crate::types::DomainEndpointOptions> {
        self.inner.get_domain_endpoint_options()
    }
    /// <p>Specifies advanced security options.</p>
    pub fn advanced_security_options(mut self, input: crate::types::AdvancedSecurityOptionsInput) -> Self {
        self.inner = self.inner.advanced_security_options(input);
        self
    }
    /// <p>Specifies advanced security options.</p>
    pub fn set_advanced_security_options(mut self, input: ::std::option::Option<crate::types::AdvancedSecurityOptionsInput>) -> Self {
        self.inner = self.inner.set_advanced_security_options(input);
        self
    }
    /// <p>Specifies advanced security options.</p>
    pub fn get_advanced_security_options(&self) -> &::std::option::Option<crate::types::AdvancedSecurityOptionsInput> {
        self.inner.get_advanced_security_options()
    }
    /// <p>Specifies Auto-Tune options.</p>
    pub fn auto_tune_options(mut self, input: crate::types::AutoTuneOptionsInput) -> Self {
        self.inner = self.inner.auto_tune_options(input);
        self
    }
    /// <p>Specifies Auto-Tune options.</p>
    pub fn set_auto_tune_options(mut self, input: ::std::option::Option<crate::types::AutoTuneOptionsInput>) -> Self {
        self.inner = self.inner.set_auto_tune_options(input);
        self
    }
    /// <p>Specifies Auto-Tune options.</p>
    pub fn get_auto_tune_options(&self) -> &::std::option::Option<crate::types::AutoTuneOptionsInput> {
        self.inner.get_auto_tune_options()
    }
    /// Appends an item to `TagList`.
    ///
    /// To override the contents of this collection use [`set_tag_list`](Self::set_tag_list).
    ///
    /// <p>A list of <code>Tag</code> added during domain creation.</p>
    pub fn tag_list(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tag_list(input);
        self
    }
    /// <p>A list of <code>Tag</code> added during domain creation.</p>
    pub fn set_tag_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tag_list(input);
        self
    }
    /// <p>A list of <code>Tag</code> added during domain creation.</p>
    pub fn get_tag_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tag_list()
    }
}
