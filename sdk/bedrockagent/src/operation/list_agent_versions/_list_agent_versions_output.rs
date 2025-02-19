// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// List Agent Versions Response
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAgentVersionsOutput {
    /// List of AgentVersionSummary.
    pub agent_version_summaries: ::std::vec::Vec<crate::types::AgentVersionSummary>,
    /// Opaque continuation token of previous paginated response.
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAgentVersionsOutput {
    /// List of AgentVersionSummary.
    pub fn agent_version_summaries(&self) -> &[crate::types::AgentVersionSummary] {
        use std::ops::Deref;
        self.agent_version_summaries.deref()
    }
    /// Opaque continuation token of previous paginated response.
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListAgentVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListAgentVersionsOutput {
    /// Creates a new builder-style object to manufacture [`ListAgentVersionsOutput`](crate::operation::list_agent_versions::ListAgentVersionsOutput).
    pub fn builder() -> crate::operation::list_agent_versions::builders::ListAgentVersionsOutputBuilder {
        crate::operation::list_agent_versions::builders::ListAgentVersionsOutputBuilder::default()
    }
}

/// A builder for [`ListAgentVersionsOutput`](crate::operation::list_agent_versions::ListAgentVersionsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListAgentVersionsOutputBuilder {
    pub(crate) agent_version_summaries: ::std::option::Option<::std::vec::Vec<crate::types::AgentVersionSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListAgentVersionsOutputBuilder {
    /// Appends an item to `agent_version_summaries`.
    ///
    /// To override the contents of this collection use [`set_agent_version_summaries`](Self::set_agent_version_summaries).
    ///
    /// List of AgentVersionSummary.
    pub fn agent_version_summaries(mut self, input: crate::types::AgentVersionSummary) -> Self {
        let mut v = self.agent_version_summaries.unwrap_or_default();
        v.push(input);
        self.agent_version_summaries = ::std::option::Option::Some(v);
        self
    }
    /// List of AgentVersionSummary.
    pub fn set_agent_version_summaries(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AgentVersionSummary>>) -> Self {
        self.agent_version_summaries = input;
        self
    }
    /// List of AgentVersionSummary.
    pub fn get_agent_version_summaries(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AgentVersionSummary>> {
        &self.agent_version_summaries
    }
    /// Opaque continuation token of previous paginated response.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Opaque continuation token of previous paginated response.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Opaque continuation token of previous paginated response.
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListAgentVersionsOutput`](crate::operation::list_agent_versions::ListAgentVersionsOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_version_summaries`](crate::operation::list_agent_versions::builders::ListAgentVersionsOutputBuilder::agent_version_summaries)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::list_agent_versions::ListAgentVersionsOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_agent_versions::ListAgentVersionsOutput {
            agent_version_summaries: self.agent_version_summaries.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_version_summaries",
                    "agent_version_summaries was not specified but it is required when building ListAgentVersionsOutput",
                )
            })?,
            next_token: self.next_token,
            _request_id: self._request_id,
        })
    }
}
