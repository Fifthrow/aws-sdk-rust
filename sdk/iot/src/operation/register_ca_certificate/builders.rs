// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_ca_certificate::_register_ca_certificate_output::RegisterCaCertificateOutputBuilder;

pub use crate::operation::register_ca_certificate::_register_ca_certificate_input::RegisterCaCertificateInputBuilder;

impl RegisterCaCertificateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::register_ca_certificate::RegisterCaCertificateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_ca_certificate::RegisterCACertificateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.register_ca_certificate();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RegisterCACertificate`.
///
/// <p>Registers a CA certificate with Amazon Web Services IoT Core. There is no limit to the number of CA certificates you can register in your Amazon Web Services account. You can register up to 10 CA certificates with the same <code>CA subject field</code> per Amazon Web Services account.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">RegisterCACertificate</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterCACertificateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_ca_certificate::builders::RegisterCaCertificateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::register_ca_certificate::RegisterCaCertificateOutput,
        crate::operation::register_ca_certificate::RegisterCACertificateError,
    > for RegisterCACertificateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::register_ca_certificate::RegisterCaCertificateOutput,
            crate::operation::register_ca_certificate::RegisterCACertificateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RegisterCACertificateFluentBuilder {
    /// Creates a new `RegisterCACertificate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RegisterCACertificate as a reference.
    pub fn as_input(&self) -> &crate::operation::register_ca_certificate::builders::RegisterCaCertificateInputBuilder {
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
        crate::operation::register_ca_certificate::RegisterCaCertificateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_ca_certificate::RegisterCACertificateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::register_ca_certificate::RegisterCACertificate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::register_ca_certificate::RegisterCACertificate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::register_ca_certificate::RegisterCaCertificateOutput,
        crate::operation::register_ca_certificate::RegisterCACertificateError,
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
    /// <p>The CA certificate.</p>
    pub fn ca_certificate(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ca_certificate(input.into());
        self
    }
    /// <p>The CA certificate.</p>
    pub fn set_ca_certificate(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ca_certificate(input);
        self
    }
    /// <p>The CA certificate.</p>
    pub fn get_ca_certificate(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ca_certificate()
    }
    /// <p>The private key verification certificate. If <code>certificateMode</code> is <code>SNI_ONLY</code>, the <code>verificationCertificate</code> field must be empty. If <code>certificateMode</code> is <code>DEFAULT</code> or not provided, the <code>verificationCertificate</code> field must not be empty.</p>
    pub fn verification_certificate(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.verification_certificate(input.into());
        self
    }
    /// <p>The private key verification certificate. If <code>certificateMode</code> is <code>SNI_ONLY</code>, the <code>verificationCertificate</code> field must be empty. If <code>certificateMode</code> is <code>DEFAULT</code> or not provided, the <code>verificationCertificate</code> field must not be empty.</p>
    pub fn set_verification_certificate(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_verification_certificate(input);
        self
    }
    /// <p>The private key verification certificate. If <code>certificateMode</code> is <code>SNI_ONLY</code>, the <code>verificationCertificate</code> field must be empty. If <code>certificateMode</code> is <code>DEFAULT</code> or not provided, the <code>verificationCertificate</code> field must not be empty.</p>
    pub fn get_verification_certificate(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_verification_certificate()
    }
    /// <p>A boolean value that specifies if the CA certificate is set to active.</p>
    /// <p>Valid values: <code>ACTIVE | INACTIVE</code></p>
    pub fn set_as_active(mut self, input: bool) -> Self {
        self.inner = self.inner.set_as_active(input);
        self
    }
    /// <p>A boolean value that specifies if the CA certificate is set to active.</p>
    /// <p>Valid values: <code>ACTIVE | INACTIVE</code></p>
    pub fn set_set_as_active(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_set_as_active(input);
        self
    }
    /// <p>A boolean value that specifies if the CA certificate is set to active.</p>
    /// <p>Valid values: <code>ACTIVE | INACTIVE</code></p>
    pub fn get_set_as_active(&self) -> &::std::option::Option<bool> {
        self.inner.get_set_as_active()
    }
    /// <p>Allows this CA certificate to be used for auto registration of device certificates.</p>
    pub fn allow_auto_registration(mut self, input: bool) -> Self {
        self.inner = self.inner.allow_auto_registration(input);
        self
    }
    /// <p>Allows this CA certificate to be used for auto registration of device certificates.</p>
    pub fn set_allow_auto_registration(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_allow_auto_registration(input);
        self
    }
    /// <p>Allows this CA certificate to be used for auto registration of device certificates.</p>
    pub fn get_allow_auto_registration(&self) -> &::std::option::Option<bool> {
        self.inner.get_allow_auto_registration()
    }
    /// <p>Information about the registration configuration.</p>
    pub fn registration_config(mut self, input: crate::types::RegistrationConfig) -> Self {
        self.inner = self.inner.registration_config(input);
        self
    }
    /// <p>Information about the registration configuration.</p>
    pub fn set_registration_config(mut self, input: ::std::option::Option<crate::types::RegistrationConfig>) -> Self {
        self.inner = self.inner.set_registration_config(input);
        self
    }
    /// <p>Information about the registration configuration.</p>
    pub fn get_registration_config(&self) -> &::std::option::Option<crate::types::RegistrationConfig> {
        self.inner.get_registration_config()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Metadata which can be used to manage the CA certificate.</p><note>
    /// <p>For URI Request parameters use format: ...key1=value1&amp;key2=value2...</p>
    /// <p>For the CLI command-line parameter use format: &amp;&amp;tags "key1=value1&amp;key2=value2..."</p>
    /// <p>For the cli-input-json file use format: "tags": "key1=value1&amp;key2=value2..."</p>
    /// </note>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Metadata which can be used to manage the CA certificate.</p><note>
    /// <p>For URI Request parameters use format: ...key1=value1&amp;key2=value2...</p>
    /// <p>For the CLI command-line parameter use format: &amp;&amp;tags "key1=value1&amp;key2=value2..."</p>
    /// <p>For the cli-input-json file use format: "tags": "key1=value1&amp;key2=value2..."</p>
    /// </note>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Metadata which can be used to manage the CA certificate.</p><note>
    /// <p>For URI Request parameters use format: ...key1=value1&amp;key2=value2...</p>
    /// <p>For the CLI command-line parameter use format: &amp;&amp;tags "key1=value1&amp;key2=value2..."</p>
    /// <p>For the cli-input-json file use format: "tags": "key1=value1&amp;key2=value2..."</p>
    /// </note>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>Describes the certificate mode in which the Certificate Authority (CA) will be registered. If the <code>verificationCertificate</code> field is not provided, set <code>certificateMode</code> to be <code>SNI_ONLY</code>. If the <code>verificationCertificate</code> field is provided, set <code>certificateMode</code> to be <code>DEFAULT</code>. When <code>certificateMode</code> is not provided, it defaults to <code>DEFAULT</code>. All the device certificates that are registered using this CA will be registered in the same certificate mode as the CA. For more information about certificate mode for device certificates, see <a href="https://docs.aws.amazon.com/iot/latest/apireference/API_CertificateDescription.html#iot-Type-CertificateDescription-certificateMode"> certificate mode</a>.</p>
    pub fn certificate_mode(mut self, input: crate::types::CertificateMode) -> Self {
        self.inner = self.inner.certificate_mode(input);
        self
    }
    /// <p>Describes the certificate mode in which the Certificate Authority (CA) will be registered. If the <code>verificationCertificate</code> field is not provided, set <code>certificateMode</code> to be <code>SNI_ONLY</code>. If the <code>verificationCertificate</code> field is provided, set <code>certificateMode</code> to be <code>DEFAULT</code>. When <code>certificateMode</code> is not provided, it defaults to <code>DEFAULT</code>. All the device certificates that are registered using this CA will be registered in the same certificate mode as the CA. For more information about certificate mode for device certificates, see <a href="https://docs.aws.amazon.com/iot/latest/apireference/API_CertificateDescription.html#iot-Type-CertificateDescription-certificateMode"> certificate mode</a>.</p>
    pub fn set_certificate_mode(mut self, input: ::std::option::Option<crate::types::CertificateMode>) -> Self {
        self.inner = self.inner.set_certificate_mode(input);
        self
    }
    /// <p>Describes the certificate mode in which the Certificate Authority (CA) will be registered. If the <code>verificationCertificate</code> field is not provided, set <code>certificateMode</code> to be <code>SNI_ONLY</code>. If the <code>verificationCertificate</code> field is provided, set <code>certificateMode</code> to be <code>DEFAULT</code>. When <code>certificateMode</code> is not provided, it defaults to <code>DEFAULT</code>. All the device certificates that are registered using this CA will be registered in the same certificate mode as the CA. For more information about certificate mode for device certificates, see <a href="https://docs.aws.amazon.com/iot/latest/apireference/API_CertificateDescription.html#iot-Type-CertificateDescription-certificateMode"> certificate mode</a>.</p>
    pub fn get_certificate_mode(&self) -> &::std::option::Option<crate::types::CertificateMode> {
        self.inner.get_certificate_mode()
    }
}
