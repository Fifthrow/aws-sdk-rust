// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_job_template::_update_job_template_output::UpdateJobTemplateOutputBuilder;

pub use crate::operation::update_job_template::_update_job_template_input::UpdateJobTemplateInputBuilder;

impl UpdateJobTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_job_template::UpdateJobTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_job_template::UpdateJobTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_job_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateJobTemplate`.
///
/// Modify one of your existing job templates.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateJobTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_job_template::builders::UpdateJobTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_job_template::UpdateJobTemplateOutput,
        crate::operation::update_job_template::UpdateJobTemplateError,
    > for UpdateJobTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_job_template::UpdateJobTemplateOutput,
            crate::operation::update_job_template::UpdateJobTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateJobTemplateFluentBuilder {
    /// Creates a new `UpdateJobTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateJobTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::update_job_template::builders::UpdateJobTemplateInputBuilder {
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
        crate::operation::update_job_template::UpdateJobTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_job_template::UpdateJobTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_job_template::UpdateJobTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_job_template::UpdateJobTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_job_template::UpdateJobTemplateOutput,
        crate::operation::update_job_template::UpdateJobTemplateError,
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
    /// Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.
    pub fn acceleration_settings(mut self, input: crate::types::AccelerationSettings) -> Self {
        self.inner = self.inner.acceleration_settings(input);
        self
    }
    /// Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.
    pub fn set_acceleration_settings(mut self, input: ::std::option::Option<crate::types::AccelerationSettings>) -> Self {
        self.inner = self.inner.set_acceleration_settings(input);
        self
    }
    /// Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.
    pub fn get_acceleration_settings(&self) -> &::std::option::Option<crate::types::AccelerationSettings> {
        self.inner.get_acceleration_settings()
    }
    /// The new category for the job template, if you are changing it.
    pub fn category(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.category(input.into());
        self
    }
    /// The new category for the job template, if you are changing it.
    pub fn set_category(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_category(input);
        self
    }
    /// The new category for the job template, if you are changing it.
    pub fn get_category(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_category()
    }
    /// The new description for the job template, if you are changing it.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// The new description for the job template, if you are changing it.
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// The new description for the job template, if you are changing it.
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `HopDestinations`.
    ///
    /// To override the contents of this collection use [`set_hop_destinations`](Self::set_hop_destinations).
    ///
    /// Optional list of hop destinations.
    pub fn hop_destinations(mut self, input: crate::types::HopDestination) -> Self {
        self.inner = self.inner.hop_destinations(input);
        self
    }
    /// Optional list of hop destinations.
    pub fn set_hop_destinations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::HopDestination>>) -> Self {
        self.inner = self.inner.set_hop_destinations(input);
        self
    }
    /// Optional list of hop destinations.
    pub fn get_hop_destinations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::HopDestination>> {
        self.inner.get_hop_destinations()
    }
    /// The name of the job template you are modifying
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// The name of the job template you are modifying
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// The name of the job template you are modifying
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don't specify a priority, the service uses the default value 0.
    pub fn priority(mut self, input: i32) -> Self {
        self.inner = self.inner.priority(input);
        self
    }
    /// Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don't specify a priority, the service uses the default value 0.
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_priority(input);
        self
    }
    /// Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don't specify a priority, the service uses the default value 0.
    pub fn get_priority(&self) -> &::std::option::Option<i32> {
        self.inner.get_priority()
    }
    /// The new queue for the job template, if you are changing it.
    pub fn queue(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue(input.into());
        self
    }
    /// The new queue for the job template, if you are changing it.
    pub fn set_queue(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue(input);
        self
    }
    /// The new queue for the job template, if you are changing it.
    pub fn get_queue(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue()
    }
    /// JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.
    pub fn settings(mut self, input: crate::types::JobTemplateSettings) -> Self {
        self.inner = self.inner.settings(input);
        self
    }
    /// JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.
    pub fn set_settings(mut self, input: ::std::option::Option<crate::types::JobTemplateSettings>) -> Self {
        self.inner = self.inner.set_settings(input);
        self
    }
    /// JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.
    pub fn get_settings(&self) -> &::std::option::Option<crate::types::JobTemplateSettings> {
        self.inner.get_settings()
    }
    /// Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.
    pub fn status_update_interval(mut self, input: crate::types::StatusUpdateInterval) -> Self {
        self.inner = self.inner.status_update_interval(input);
        self
    }
    /// Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.
    pub fn set_status_update_interval(mut self, input: ::std::option::Option<crate::types::StatusUpdateInterval>) -> Self {
        self.inner = self.inner.set_status_update_interval(input);
        self
    }
    /// Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.
    pub fn get_status_update_interval(&self) -> &::std::option::Option<crate::types::StatusUpdateInterval> {
        self.inner.get_status_update_interval()
    }
}
