// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_account_settings::_update_account_settings_output::UpdateAccountSettingsOutputBuilder;

pub use crate::operation::update_account_settings::_update_account_settings_input::UpdateAccountSettingsInputBuilder;

impl UpdateAccountSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_account_settings::UpdateAccountSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_account_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateAccountSettings`.
///
/// <p>Update Proton settings that are used for multiple services in the Amazon Web Services account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateAccountSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_account_settings::builders::UpdateAccountSettingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_account_settings::UpdateAccountSettingsOutput,
        crate::operation::update_account_settings::UpdateAccountSettingsError,
    > for UpdateAccountSettingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_account_settings::UpdateAccountSettingsOutput,
            crate::operation::update_account_settings::UpdateAccountSettingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateAccountSettingsFluentBuilder {
    /// Creates a new `UpdateAccountSettings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateAccountSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::update_account_settings::builders::UpdateAccountSettingsInputBuilder {
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
        crate::operation::update_account_settings::UpdateAccountSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_account_settings::UpdateAccountSettings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_account_settings::UpdateAccountSettings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_account_settings::UpdateAccountSettingsOutput,
        crate::operation::update_account_settings::UpdateAccountSettingsError,
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
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Assumed by Proton for Amazon Web Services-managed provisioning, and by customer-owned automation for self-managed provisioning.</p>
    /// <p>To remove a previously configured ARN, specify an empty string.</p>
    pub fn pipeline_service_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pipeline_service_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Assumed by Proton for Amazon Web Services-managed provisioning, and by customer-owned automation for self-managed provisioning.</p>
    /// <p>To remove a previously configured ARN, specify an empty string.</p>
    pub fn set_pipeline_service_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pipeline_service_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Assumed by Proton for Amazon Web Services-managed provisioning, and by customer-owned automation for self-managed provisioning.</p>
    /// <p>To remove a previously configured ARN, specify an empty string.</p>
    pub fn get_pipeline_service_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pipeline_service_role_arn()
    }
    /// <p>A linked repository for pipeline provisioning. Specify it if you have environments configured for self-managed provisioning with services that include pipelines. A linked repository is a repository that has been registered with Proton. For more information, see <code>CreateRepository</code>.</p>
    /// <p>To remove a previously configured repository, set <code>deletePipelineProvisioningRepository</code> to <code>true</code>, and don't set <code>pipelineProvisioningRepository</code>.</p>
    pub fn pipeline_provisioning_repository(mut self, input: crate::types::RepositoryBranchInput) -> Self {
        self.inner = self.inner.pipeline_provisioning_repository(input);
        self
    }
    /// <p>A linked repository for pipeline provisioning. Specify it if you have environments configured for self-managed provisioning with services that include pipelines. A linked repository is a repository that has been registered with Proton. For more information, see <code>CreateRepository</code>.</p>
    /// <p>To remove a previously configured repository, set <code>deletePipelineProvisioningRepository</code> to <code>true</code>, and don't set <code>pipelineProvisioningRepository</code>.</p>
    pub fn set_pipeline_provisioning_repository(mut self, input: ::std::option::Option<crate::types::RepositoryBranchInput>) -> Self {
        self.inner = self.inner.set_pipeline_provisioning_repository(input);
        self
    }
    /// <p>A linked repository for pipeline provisioning. Specify it if you have environments configured for self-managed provisioning with services that include pipelines. A linked repository is a repository that has been registered with Proton. For more information, see <code>CreateRepository</code>.</p>
    /// <p>To remove a previously configured repository, set <code>deletePipelineProvisioningRepository</code> to <code>true</code>, and don't set <code>pipelineProvisioningRepository</code>.</p>
    pub fn get_pipeline_provisioning_repository(&self) -> &::std::option::Option<crate::types::RepositoryBranchInput> {
        self.inner.get_pipeline_provisioning_repository()
    }
    /// <p>Set to <code>true</code> to remove a configured pipeline repository from the account settings. Don't set this field if you are updating the configured pipeline repository.</p>
    pub fn delete_pipeline_provisioning_repository(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_pipeline_provisioning_repository(input);
        self
    }
    /// <p>Set to <code>true</code> to remove a configured pipeline repository from the account settings. Don't set this field if you are updating the configured pipeline repository.</p>
    pub fn set_delete_pipeline_provisioning_repository(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_pipeline_provisioning_repository(input);
        self
    }
    /// <p>Set to <code>true</code> to remove a configured pipeline repository from the account settings. Don't set this field if you are updating the configured pipeline repository.</p>
    pub fn get_delete_pipeline_provisioning_repository(&self) -> &::std::option::Option<bool> {
        self.inner.get_delete_pipeline_provisioning_repository()
    }
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Proton assumes this role for CodeBuild-based provisioning.</p>
    pub fn pipeline_codebuild_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pipeline_codebuild_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Proton assumes this role for CodeBuild-based provisioning.</p>
    pub fn set_pipeline_codebuild_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pipeline_codebuild_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Proton assumes this role for CodeBuild-based provisioning.</p>
    pub fn get_pipeline_codebuild_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pipeline_codebuild_role_arn()
    }
}
