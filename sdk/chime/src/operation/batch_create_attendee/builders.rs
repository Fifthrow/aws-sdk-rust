// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_create_attendee::_batch_create_attendee_output::BatchCreateAttendeeOutputBuilder;

pub use crate::operation::batch_create_attendee::_batch_create_attendee_input::BatchCreateAttendeeInputBuilder;

impl BatchCreateAttendeeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_create_attendee::BatchCreateAttendeeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_create_attendee::BatchCreateAttendeeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_create_attendee();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchCreateAttendee`.
///
/// <p>Creates up to 100 new attendees for an active Amazon Chime SDK meeting.</p><important>
/// <p><b>This API is is no longer supported and will not be updated.</b> We recommend using the latest version, <a href="https://docs.aws.amazon.com/chime-sdk/latest/APIReference/API_meeting-chime_BatchCreateAttendee.html">BatchCreateAttendee</a>, in the Amazon Chime SDK.</p>
/// <p>Using the latest version requires migrating to a dedicated namespace. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/migrate-from-chm-namespace.html">Migrating from the Amazon Chime namespace</a> in the <i>Amazon Chime SDK Developer Guide</i>.</p>
/// </important>
/// <p>For more information about the Amazon Chime SDK, see <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/meetings-sdk.html">Using the Amazon Chime SDK</a> in the <i>Amazon Chime SDK Developer Guide</i>.</p>
#[deprecated(note = "Replaced by BatchCreateAttendee in the Amazon Chime SDK Meetings Namespace")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchCreateAttendeeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_create_attendee::builders::BatchCreateAttendeeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_create_attendee::BatchCreateAttendeeOutput,
        crate::operation::batch_create_attendee::BatchCreateAttendeeError,
    > for BatchCreateAttendeeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_create_attendee::BatchCreateAttendeeOutput,
            crate::operation::batch_create_attendee::BatchCreateAttendeeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchCreateAttendeeFluentBuilder {
    /// Creates a new `BatchCreateAttendee`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchCreateAttendee as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_create_attendee::builders::BatchCreateAttendeeInputBuilder {
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
        crate::operation::batch_create_attendee::BatchCreateAttendeeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_create_attendee::BatchCreateAttendeeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_create_attendee::BatchCreateAttendee::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_create_attendee::BatchCreateAttendee::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_create_attendee::BatchCreateAttendeeOutput,
        crate::operation::batch_create_attendee::BatchCreateAttendeeError,
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
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.meeting_id(input.into());
        self
    }
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn set_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_meeting_id(input);
        self
    }
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn get_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_meeting_id()
    }
    /// Appends an item to `Attendees`.
    ///
    /// To override the contents of this collection use [`set_attendees`](Self::set_attendees).
    ///
    /// <p>The request containing the attendees to create.</p>
    pub fn attendees(mut self, input: crate::types::CreateAttendeeRequestItem) -> Self {
        self.inner = self.inner.attendees(input);
        self
    }
    /// <p>The request containing the attendees to create.</p>
    pub fn set_attendees(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>>) -> Self {
        self.inner = self.inner.set_attendees(input);
        self
    }
    /// <p>The request containing the attendees to create.</p>
    pub fn get_attendees(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CreateAttendeeRequestItem>> {
        self.inner.get_attendees()
    }
}
