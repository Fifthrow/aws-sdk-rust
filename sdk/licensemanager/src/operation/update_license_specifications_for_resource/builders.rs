// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_license_specifications_for_resource::_update_license_specifications_for_resource_output::UpdateLicenseSpecificationsForResourceOutputBuilder;

pub use crate::operation::update_license_specifications_for_resource::_update_license_specifications_for_resource_input::UpdateLicenseSpecificationsForResourceInputBuilder;

impl UpdateLicenseSpecificationsForResourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_license_specifications_for_resource();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateLicenseSpecificationsForResource`.
///
/// <p>Adds or removes the specified license configurations for the specified Amazon Web Services resource.</p>
/// <p>You can update the license specifications of AMIs, instances, and hosts. You cannot update the license specifications for launch templates and CloudFormation templates, as they send license configurations to the operation that creates the resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLicenseSpecificationsForResourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_license_specifications_for_resource::builders::UpdateLicenseSpecificationsForResourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceOutput,
        crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceError,
    > for UpdateLicenseSpecificationsForResourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceOutput,
            crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateLicenseSpecificationsForResourceFluentBuilder {
    /// Creates a new `UpdateLicenseSpecificationsForResource`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateLicenseSpecificationsForResource as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::update_license_specifications_for_resource::builders::UpdateLicenseSpecificationsForResourceInputBuilder {
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
        crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResource::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResource::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceOutput,
        crate::operation::update_license_specifications_for_resource::UpdateLicenseSpecificationsForResourceError,
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
    /// <p>Amazon Resource Name (ARN) of the Amazon Web Services resource.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the Amazon Web Services resource.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>Amazon Resource Name (ARN) of the Amazon Web Services resource.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
    /// Appends an item to `AddLicenseSpecifications`.
    ///
    /// To override the contents of this collection use [`set_add_license_specifications`](Self::set_add_license_specifications).
    ///
    /// <p>ARNs of the license configurations to add.</p>
    pub fn add_license_specifications(mut self, input: crate::types::LicenseSpecification) -> Self {
        self.inner = self.inner.add_license_specifications(input);
        self
    }
    /// <p>ARNs of the license configurations to add.</p>
    pub fn set_add_license_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LicenseSpecification>>) -> Self {
        self.inner = self.inner.set_add_license_specifications(input);
        self
    }
    /// <p>ARNs of the license configurations to add.</p>
    pub fn get_add_license_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LicenseSpecification>> {
        self.inner.get_add_license_specifications()
    }
    /// Appends an item to `RemoveLicenseSpecifications`.
    ///
    /// To override the contents of this collection use [`set_remove_license_specifications`](Self::set_remove_license_specifications).
    ///
    /// <p>ARNs of the license configurations to remove.</p>
    pub fn remove_license_specifications(mut self, input: crate::types::LicenseSpecification) -> Self {
        self.inner = self.inner.remove_license_specifications(input);
        self
    }
    /// <p>ARNs of the license configurations to remove.</p>
    pub fn set_remove_license_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LicenseSpecification>>) -> Self {
        self.inner = self.inner.set_remove_license_specifications(input);
        self
    }
    /// <p>ARNs of the license configurations to remove.</p>
    pub fn get_remove_license_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LicenseSpecification>> {
        self.inner.get_remove_license_specifications()
    }
}
