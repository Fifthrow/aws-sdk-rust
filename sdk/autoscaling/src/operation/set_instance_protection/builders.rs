// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_instance_protection::_set_instance_protection_output::SetInstanceProtectionOutputBuilder;

pub use crate::operation::set_instance_protection::_set_instance_protection_input::SetInstanceProtectionInputBuilder;

impl SetInstanceProtectionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_instance_protection::SetInstanceProtectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_instance_protection::SetInstanceProtectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_instance_protection();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetInstanceProtection`.
///
/// <p>Updates the instance protection settings of the specified instances. This operation cannot be called on instances in a warm pool.</p>
/// <p>For more information about preventing instances that are part of an Auto Scaling group from terminating on scale in, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-instance-protection.html">Using instance scale-in protection</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
/// <p>If you exceed your maximum limit of instance IDs, which is 50 per Auto Scaling group, the call fails.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetInstanceProtectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_instance_protection::builders::SetInstanceProtectionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_instance_protection::SetInstanceProtectionOutput,
        crate::operation::set_instance_protection::SetInstanceProtectionError,
    > for SetInstanceProtectionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_instance_protection::SetInstanceProtectionOutput,
            crate::operation::set_instance_protection::SetInstanceProtectionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetInstanceProtectionFluentBuilder {
    /// Creates a new `SetInstanceProtection`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetInstanceProtection as a reference.
    pub fn as_input(&self) -> &crate::operation::set_instance_protection::builders::SetInstanceProtectionInputBuilder {
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
        crate::operation::set_instance_protection::SetInstanceProtectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_instance_protection::SetInstanceProtectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::set_instance_protection::SetInstanceProtection::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::set_instance_protection::SetInstanceProtection::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_instance_protection::SetInstanceProtectionOutput,
        crate::operation::set_instance_protection::SetInstanceProtectionError,
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
    /// Appends an item to `InstanceIds`.
    ///
    /// To override the contents of this collection use [`set_instance_ids`](Self::set_instance_ids).
    ///
    /// <p>One or more instance IDs. You can specify up to 50 instances.</p>
    pub fn instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_ids(input.into());
        self
    }
    /// <p>One or more instance IDs. You can specify up to 50 instances.</p>
    pub fn set_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_instance_ids(input);
        self
    }
    /// <p>One or more instance IDs. You can specify up to 50 instances.</p>
    pub fn get_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_instance_ids()
    }
    /// <p>The name of the Auto Scaling group.</p>
    pub fn auto_scaling_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.auto_scaling_group_name(input.into());
        self
    }
    /// <p>The name of the Auto Scaling group.</p>
    pub fn set_auto_scaling_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_auto_scaling_group_name(input);
        self
    }
    /// <p>The name of the Auto Scaling group.</p>
    pub fn get_auto_scaling_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_auto_scaling_group_name()
    }
    /// <p>Indicates whether the instance is protected from termination by Amazon EC2 Auto Scaling when scaling in.</p>
    pub fn protected_from_scale_in(mut self, input: bool) -> Self {
        self.inner = self.inner.protected_from_scale_in(input);
        self
    }
    /// <p>Indicates whether the instance is protected from termination by Amazon EC2 Auto Scaling when scaling in.</p>
    pub fn set_protected_from_scale_in(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_protected_from_scale_in(input);
        self
    }
    /// <p>Indicates whether the instance is protected from termination by Amazon EC2 Auto Scaling when scaling in.</p>
    pub fn get_protected_from_scale_in(&self) -> &::std::option::Option<bool> {
        self.inner.get_protected_from_scale_in()
    }
}
