// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_meeting_transcription::_start_meeting_transcription_output::StartMeetingTranscriptionOutputBuilder;

pub use crate::operation::start_meeting_transcription::_start_meeting_transcription_input::StartMeetingTranscriptionInputBuilder;

impl StartMeetingTranscriptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_meeting_transcription::StartMeetingTranscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_meeting_transcription::StartMeetingTranscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_meeting_transcription();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartMeetingTranscription`.
///
/// <p>Starts transcription for the specified <code>meetingId</code>. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/meeting-transcription.html"> Using Amazon Chime SDK live transcription </a> in the <i>Amazon Chime SDK Developer Guide</i>.</p>
/// <p>If you specify an invalid configuration, a <code>TranscriptFailed</code> event will be sent with the contents of the <code>BadRequestException</code> generated by Amazon Transcribe. For more information on each parameter and which combinations are valid, refer to the <a href="https://docs.aws.amazon.com/transcribe/latest/APIReference/API_streaming_StartStreamTranscription.html">StartStreamTranscription</a> API in the <i>Amazon Transcribe Developer Guide</i>.</p><note>
/// <p>By default, Amazon Transcribe may use and store audio content processed by the service to develop and improve Amazon Web Services AI/ML services as further described in section 50 of the <a href="https://aws.amazon.com/service-terms/">Amazon Web Services Service Terms</a>. Using Amazon Transcribe may be subject to federal and state laws or regulations regarding the recording or interception of electronic communications. It is your and your end users’ responsibility to comply with all applicable laws regarding the recording, including properly notifying all participants in a recorded session or communication that the session or communication is being recorded, and obtaining all necessary consents. You can opt out from Amazon Web Services using audio content to develop and improve AWS AI/ML services by configuring an AI services opt out policy using Amazon Web Services Organizations.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartMeetingTranscriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_meeting_transcription::builders::StartMeetingTranscriptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_meeting_transcription::StartMeetingTranscriptionOutput,
        crate::operation::start_meeting_transcription::StartMeetingTranscriptionError,
    > for StartMeetingTranscriptionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_meeting_transcription::StartMeetingTranscriptionOutput,
            crate::operation::start_meeting_transcription::StartMeetingTranscriptionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartMeetingTranscriptionFluentBuilder {
    /// Creates a new `StartMeetingTranscription`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartMeetingTranscription as a reference.
    pub fn as_input(&self) -> &crate::operation::start_meeting_transcription::builders::StartMeetingTranscriptionInputBuilder {
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
        crate::operation::start_meeting_transcription::StartMeetingTranscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_meeting_transcription::StartMeetingTranscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_meeting_transcription::StartMeetingTranscription::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_meeting_transcription::StartMeetingTranscription::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_meeting_transcription::StartMeetingTranscriptionOutput,
        crate::operation::start_meeting_transcription::StartMeetingTranscriptionError,
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
    /// <p>The unique ID of the meeting being transcribed.</p>
    pub fn meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.meeting_id(input.into());
        self
    }
    /// <p>The unique ID of the meeting being transcribed.</p>
    pub fn set_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_meeting_id(input);
        self
    }
    /// <p>The unique ID of the meeting being transcribed.</p>
    pub fn get_meeting_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_meeting_id()
    }
    /// <p>The configuration for the current transcription operation. Must contain <code>EngineTranscribeSettings</code> or <code>EngineTranscribeMedicalSettings</code>.</p>
    pub fn transcription_configuration(mut self, input: crate::types::TranscriptionConfiguration) -> Self {
        self.inner = self.inner.transcription_configuration(input);
        self
    }
    /// <p>The configuration for the current transcription operation. Must contain <code>EngineTranscribeSettings</code> or <code>EngineTranscribeMedicalSettings</code>.</p>
    pub fn set_transcription_configuration(mut self, input: ::std::option::Option<crate::types::TranscriptionConfiguration>) -> Self {
        self.inner = self.inner.set_transcription_configuration(input);
        self
    }
    /// <p>The configuration for the current transcription operation. Must contain <code>EngineTranscribeSettings</code> or <code>EngineTranscribeMedicalSettings</code>.</p>
    pub fn get_transcription_configuration(&self) -> &::std::option::Option<crate::types::TranscriptionConfiguration> {
        self.inner.get_transcription_configuration()
    }
}
