// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_service_software_update::_start_service_software_update_output::StartServiceSoftwareUpdateOutputBuilder;

pub use crate::operation::start_service_software_update::_start_service_software_update_input::StartServiceSoftwareUpdateInputBuilder;

impl StartServiceSoftwareUpdateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_service_software_update::StartServiceSoftwareUpdateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_service_software_update::StartServiceSoftwareUpdateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_service_software_update();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartServiceSoftwareUpdate`.
///
/// <p>Schedules a service software update for an Amazon OpenSearch Service domain. For more information, see <a href="https://docs.aws.amazon.com/opensearch-service/latest/developerguide/service-software.html">Service software updates in Amazon OpenSearch Service</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartServiceSoftwareUpdateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_service_software_update::builders::StartServiceSoftwareUpdateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_service_software_update::StartServiceSoftwareUpdateOutput,
        crate::operation::start_service_software_update::StartServiceSoftwareUpdateError,
    > for StartServiceSoftwareUpdateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_service_software_update::StartServiceSoftwareUpdateOutput,
            crate::operation::start_service_software_update::StartServiceSoftwareUpdateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartServiceSoftwareUpdateFluentBuilder {
    /// Creates a new `StartServiceSoftwareUpdate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartServiceSoftwareUpdate as a reference.
    pub fn as_input(&self) -> &crate::operation::start_service_software_update::builders::StartServiceSoftwareUpdateInputBuilder {
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
        crate::operation::start_service_software_update::StartServiceSoftwareUpdateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_service_software_update::StartServiceSoftwareUpdateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_service_software_update::StartServiceSoftwareUpdate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_service_software_update::StartServiceSoftwareUpdate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_service_software_update::StartServiceSoftwareUpdateOutput,
        crate::operation::start_service_software_update::StartServiceSoftwareUpdateError,
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
    /// <p>The name of the domain that you want to update to the latest service software.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain that you want to update to the latest service software.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the domain that you want to update to the latest service software.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>When to start the service software update.</p>
    /// <ul>
    /// <li>
    /// <p><code>NOW</code> - Immediately schedules the update to happen in the current hour if there's capacity available.</p></li>
    /// <li>
    /// <p><code>TIMESTAMP</code> - Lets you specify a custom date and time to apply the update. If you specify this value, you must also provide a value for <code>DesiredStartTime</code>.</p></li>
    /// <li>
    /// <p><code>OFF_PEAK_WINDOW</code> - Marks the update to be picked up during an upcoming off-peak window. There's no guarantee that the update will happen during the next immediate window. Depending on capacity, it might happen in subsequent days.</p></li>
    /// </ul>
    /// <p>Default: <code>NOW</code> if you don't specify a value for <code>DesiredStartTime</code>, and <code>TIMESTAMP</code> if you do.</p>
    pub fn schedule_at(mut self, input: crate::types::ScheduleAt) -> Self {
        self.inner = self.inner.schedule_at(input);
        self
    }
    /// <p>When to start the service software update.</p>
    /// <ul>
    /// <li>
    /// <p><code>NOW</code> - Immediately schedules the update to happen in the current hour if there's capacity available.</p></li>
    /// <li>
    /// <p><code>TIMESTAMP</code> - Lets you specify a custom date and time to apply the update. If you specify this value, you must also provide a value for <code>DesiredStartTime</code>.</p></li>
    /// <li>
    /// <p><code>OFF_PEAK_WINDOW</code> - Marks the update to be picked up during an upcoming off-peak window. There's no guarantee that the update will happen during the next immediate window. Depending on capacity, it might happen in subsequent days.</p></li>
    /// </ul>
    /// <p>Default: <code>NOW</code> if you don't specify a value for <code>DesiredStartTime</code>, and <code>TIMESTAMP</code> if you do.</p>
    pub fn set_schedule_at(mut self, input: ::std::option::Option<crate::types::ScheduleAt>) -> Self {
        self.inner = self.inner.set_schedule_at(input);
        self
    }
    /// <p>When to start the service software update.</p>
    /// <ul>
    /// <li>
    /// <p><code>NOW</code> - Immediately schedules the update to happen in the current hour if there's capacity available.</p></li>
    /// <li>
    /// <p><code>TIMESTAMP</code> - Lets you specify a custom date and time to apply the update. If you specify this value, you must also provide a value for <code>DesiredStartTime</code>.</p></li>
    /// <li>
    /// <p><code>OFF_PEAK_WINDOW</code> - Marks the update to be picked up during an upcoming off-peak window. There's no guarantee that the update will happen during the next immediate window. Depending on capacity, it might happen in subsequent days.</p></li>
    /// </ul>
    /// <p>Default: <code>NOW</code> if you don't specify a value for <code>DesiredStartTime</code>, and <code>TIMESTAMP</code> if you do.</p>
    pub fn get_schedule_at(&self) -> &::std::option::Option<crate::types::ScheduleAt> {
        self.inner.get_schedule_at()
    }
    /// <p>The Epoch timestamp when you want the service software update to start. You only need to specify this parameter if you set <code>ScheduleAt</code> to <code>TIMESTAMP</code>.</p>
    pub fn desired_start_time(mut self, input: i64) -> Self {
        self.inner = self.inner.desired_start_time(input);
        self
    }
    /// <p>The Epoch timestamp when you want the service software update to start. You only need to specify this parameter if you set <code>ScheduleAt</code> to <code>TIMESTAMP</code>.</p>
    pub fn set_desired_start_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_desired_start_time(input);
        self
    }
    /// <p>The Epoch timestamp when you want the service software update to start. You only need to specify this parameter if you set <code>ScheduleAt</code> to <code>TIMESTAMP</code>.</p>
    pub fn get_desired_start_time(&self) -> &::std::option::Option<i64> {
        self.inner.get_desired_start_time()
    }
}
