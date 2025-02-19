// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Update Agent Response
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAgentOutput {
    /// Contains the information of an agent
    pub agent: ::std::option::Option<crate::types::Agent>,
    _request_id: Option<String>,
}
impl UpdateAgentOutput {
    /// Contains the information of an agent
    pub fn agent(&self) -> ::std::option::Option<&crate::types::Agent> {
        self.agent.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for UpdateAgentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateAgentOutput {
    /// Creates a new builder-style object to manufacture [`UpdateAgentOutput`](crate::operation::update_agent::UpdateAgentOutput).
    pub fn builder() -> crate::operation::update_agent::builders::UpdateAgentOutputBuilder {
        crate::operation::update_agent::builders::UpdateAgentOutputBuilder::default()
    }
}

/// A builder for [`UpdateAgentOutput`](crate::operation::update_agent::UpdateAgentOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateAgentOutputBuilder {
    pub(crate) agent: ::std::option::Option<crate::types::Agent>,
    _request_id: Option<String>,
}
impl UpdateAgentOutputBuilder {
    /// Contains the information of an agent
    /// This field is required.
    pub fn agent(mut self, input: crate::types::Agent) -> Self {
        self.agent = ::std::option::Option::Some(input);
        self
    }
    /// Contains the information of an agent
    pub fn set_agent(mut self, input: ::std::option::Option<crate::types::Agent>) -> Self {
        self.agent = input;
        self
    }
    /// Contains the information of an agent
    pub fn get_agent(&self) -> &::std::option::Option<crate::types::Agent> {
        &self.agent
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAgentOutput`](crate::operation::update_agent::UpdateAgentOutput).
    pub fn build(self) -> crate::operation::update_agent::UpdateAgentOutput {
        crate::operation::update_agent::UpdateAgentOutput {
            agent: self.agent,
            _request_id: self._request_id,
        }
    }
}
