// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_rate_based_rule::_create_rate_based_rule_output::CreateRateBasedRuleOutputBuilder;

pub use crate::operation::create_rate_based_rule::_create_rate_based_rule_input::CreateRateBasedRuleInputBuilder;

impl CreateRateBasedRuleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_rate_based_rule::CreateRateBasedRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_rate_based_rule::CreateRateBasedRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_rate_based_rule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRateBasedRule`.
///
/// <note>
/// <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p>
/// <p><b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use.</p>
/// </note>
/// <p>Creates a <code>RateBasedRule</code>. The <code>RateBasedRule</code> contains a <code>RateLimit</code>, which specifies the maximum number of requests that AWS WAF allows from a specified IP address in a five-minute period. The <code>RateBasedRule</code> also contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to count or block if these requests exceed the <code>RateLimit</code>.</p>
/// <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request not only must exceed the <code>RateLimit</code>, but it also must match all the conditions to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p>
/// <ul>
/// <li>
/// <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code></p></li>
/// <li>
/// <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p></li>
/// </ul>
/// <p>Further, you specify a <code>RateLimit</code> of 1,000.</p>
/// <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that meet the conditions in the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions must be received at a rate of more than 1,000 requests every five minutes. If both conditions are met and the rate is exceeded, AWS WAF blocks the requests. If the rate drops below 1,000 for a five-minute period, AWS WAF no longer blocks the requests.</p>
/// <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p>
/// <ul>
/// <li>
/// <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code></p></li>
/// <li>
/// <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code></p></li>
/// <li>
/// <p>A <code>TargetString</code> of <code>login</code></p></li>
/// </ul>
/// <p>Further, you specify a <code>RateLimit</code> of 1,000.</p>
/// <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p>
/// <p>To create and configure a <code>RateBasedRule</code>, perform the following steps:</p>
/// <ol>
/// <li>
/// <p>Create and update the predicates that you want to include in the rule. For more information, see <code>CreateByteMatchSet</code>, <code>CreateIPSet</code>, and <code>CreateSqlInjectionMatchSet</code>.</p></li>
/// <li>
/// <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p></li>
/// <li>
/// <p>Submit a <code>CreateRateBasedRule</code> request.</p></li>
/// <li>
/// <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRule</code> request.</p></li>
/// <li>
/// <p>Submit an <code>UpdateRateBasedRule</code> request to specify the predicates that you want to include in the rule.</p></li>
/// <li>
/// <p>Create and update a <code>WebACL</code> that contains the <code>RateBasedRule</code>. For more information, see <code>CreateWebACL</code>.</p></li>
/// </ol>
/// <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRateBasedRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_rate_based_rule::builders::CreateRateBasedRuleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_rate_based_rule::CreateRateBasedRuleOutput,
        crate::operation::create_rate_based_rule::CreateRateBasedRuleError,
    > for CreateRateBasedRuleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_rate_based_rule::CreateRateBasedRuleOutput,
            crate::operation::create_rate_based_rule::CreateRateBasedRuleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateRateBasedRuleFluentBuilder {
    /// Creates a new `CreateRateBasedRule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateRateBasedRule as a reference.
    pub fn as_input(&self) -> &crate::operation::create_rate_based_rule::builders::CreateRateBasedRuleInputBuilder {
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
        crate::operation::create_rate_based_rule::CreateRateBasedRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_rate_based_rule::CreateRateBasedRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_rate_based_rule::CreateRateBasedRule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_rate_based_rule::CreateRateBasedRule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_rate_based_rule::CreateRateBasedRuleOutput,
        crate::operation::create_rate_based_rule::CreateRateBasedRuleError,
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
    /// <p>A friendly name or description of the <code>RateBasedRule</code>. You can't change the name of a <code>RateBasedRule</code> after you create it.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A friendly name or description of the <code>RateBasedRule</code>. You can't change the name of a <code>RateBasedRule</code> after you create it.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A friendly name or description of the <code>RateBasedRule</code>. You can't change the name of a <code>RateBasedRule</code> after you create it.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A friendly name or description for the metrics for this <code>RateBasedRule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RateBasedRule</code>.</p>
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metric_name(input.into());
        self
    }
    /// <p>A friendly name or description for the metrics for this <code>RateBasedRule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RateBasedRule</code>.</p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metric_name(input);
        self
    }
    /// <p>A friendly name or description for the metrics for this <code>RateBasedRule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RateBasedRule</code>.</p>
    pub fn get_metric_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_metric_name()
    }
    /// <p>The field that AWS WAF uses to determine if requests are likely arriving from a single source and thus subject to rate monitoring. The only valid value for <code>RateKey</code> is <code>IP</code>. <code>IP</code> indicates that requests that arrive from the same IP address are subject to the <code>RateLimit</code> that is specified in the <code>RateBasedRule</code>.</p>
    pub fn rate_key(mut self, input: crate::types::RateKey) -> Self {
        self.inner = self.inner.rate_key(input);
        self
    }
    /// <p>The field that AWS WAF uses to determine if requests are likely arriving from a single source and thus subject to rate monitoring. The only valid value for <code>RateKey</code> is <code>IP</code>. <code>IP</code> indicates that requests that arrive from the same IP address are subject to the <code>RateLimit</code> that is specified in the <code>RateBasedRule</code>.</p>
    pub fn set_rate_key(mut self, input: ::std::option::Option<crate::types::RateKey>) -> Self {
        self.inner = self.inner.set_rate_key(input);
        self
    }
    /// <p>The field that AWS WAF uses to determine if requests are likely arriving from a single source and thus subject to rate monitoring. The only valid value for <code>RateKey</code> is <code>IP</code>. <code>IP</code> indicates that requests that arrive from the same IP address are subject to the <code>RateLimit</code> that is specified in the <code>RateBasedRule</code>.</p>
    pub fn get_rate_key(&self) -> &::std::option::Option<crate::types::RateKey> {
        self.inner.get_rate_key()
    }
    /// <p>The maximum number of requests, which have an identical value in the field that is specified by <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>
    pub fn rate_limit(mut self, input: i64) -> Self {
        self.inner = self.inner.rate_limit(input);
        self
    }
    /// <p>The maximum number of requests, which have an identical value in the field that is specified by <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>
    pub fn set_rate_limit(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_rate_limit(input);
        self
    }
    /// <p>The maximum number of requests, which have an identical value in the field that is specified by <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>
    pub fn get_rate_limit(&self) -> &::std::option::Option<i64> {
        self.inner.get_rate_limit()
    }
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <code>GetChangeTokenStatus</code>.</p>
    pub fn change_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.change_token(input.into());
        self
    }
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <code>GetChangeTokenStatus</code>.</p>
    pub fn set_change_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_change_token(input);
        self
    }
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <code>GetChangeTokenStatus</code>.</p>
    pub fn get_change_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_change_token()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p></p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p></p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p></p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
