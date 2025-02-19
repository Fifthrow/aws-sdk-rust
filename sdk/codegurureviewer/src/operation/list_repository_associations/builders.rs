// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_repository_associations::_list_repository_associations_output::ListRepositoryAssociationsOutputBuilder;

pub use crate::operation::list_repository_associations::_list_repository_associations_input::ListRepositoryAssociationsInputBuilder;

impl ListRepositoryAssociationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_repository_associations::ListRepositoryAssociationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_repository_associations::ListRepositoryAssociationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_repository_associations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListRepositoryAssociations`.
///
/// <p>Returns a list of <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html">RepositoryAssociationSummary</a> objects that contain summary information about a repository association. You can filter the returned list by <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-ProviderType">ProviderType</a>, <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-Name">Name</a>, <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-State">State</a>, and <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-api/API_RepositoryAssociationSummary.html#reviewer-Type-RepositoryAssociationSummary-Owner">Owner</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListRepositoryAssociationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_repository_associations::builders::ListRepositoryAssociationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_repository_associations::ListRepositoryAssociationsOutput,
        crate::operation::list_repository_associations::ListRepositoryAssociationsError,
    > for ListRepositoryAssociationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_repository_associations::ListRepositoryAssociationsOutput,
            crate::operation::list_repository_associations::ListRepositoryAssociationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListRepositoryAssociationsFluentBuilder {
    /// Creates a new `ListRepositoryAssociations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListRepositoryAssociations as a reference.
    pub fn as_input(&self) -> &crate::operation::list_repository_associations::builders::ListRepositoryAssociationsInputBuilder {
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
        crate::operation::list_repository_associations::ListRepositoryAssociationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_repository_associations::ListRepositoryAssociationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_repository_associations::ListRepositoryAssociations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_repository_associations::ListRepositoryAssociations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_repository_associations::ListRepositoryAssociationsOutput,
        crate::operation::list_repository_associations::ListRepositoryAssociationsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_repository_associations::paginator::ListRepositoryAssociationsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_repository_associations::paginator::ListRepositoryAssociationsPaginator {
        crate::operation::list_repository_associations::paginator::ListRepositoryAssociationsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `ProviderTypes`.
    ///
    /// To override the contents of this collection use [`set_provider_types`](Self::set_provider_types).
    ///
    /// <p>List of provider types to use as a filter.</p>
    pub fn provider_types(mut self, input: crate::types::ProviderType) -> Self {
        self.inner = self.inner.provider_types(input);
        self
    }
    /// <p>List of provider types to use as a filter.</p>
    pub fn set_provider_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProviderType>>) -> Self {
        self.inner = self.inner.set_provider_types(input);
        self
    }
    /// <p>List of provider types to use as a filter.</p>
    pub fn get_provider_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ProviderType>> {
        self.inner.get_provider_types()
    }
    /// Appends an item to `States`.
    ///
    /// To override the contents of this collection use [`set_states`](Self::set_states).
    ///
    /// <p>List of repository association states to use as a filter.</p>
    /// <p>The valid repository association states are:</p>
    /// <ul>
    /// <li>
    /// <p><b>Associated</b>: The repository association is complete.</p></li>
    /// <li>
    /// <p><b>Associating</b>: CodeGuru Reviewer is:</p>
    /// <ul>
    /// <li>
    /// <p>Setting up pull request notifications. This is required for pull requests to trigger a CodeGuru Reviewer review.</p><note>
    /// <p>If your repository <code>ProviderType</code> is <code>GitHub</code>, <code>GitHub Enterprise Server</code>, or <code>Bitbucket</code>, CodeGuru Reviewer creates webhooks in your repository to trigger CodeGuru Reviewer reviews. If you delete these webhooks, reviews of code in your repository cannot be triggered.</p>
    /// </note></li>
    /// <li>
    /// <p>Setting up source code access. This is required for CodeGuru Reviewer to securely clone code in your repository.</p></li>
    /// </ul></li>
    /// <li>
    /// <p><b>Failed</b>: The repository failed to associate or disassociate.</p></li>
    /// <li>
    /// <p><b>Disassociating</b>: CodeGuru Reviewer is removing the repository's pull request notifications and source code access.</p></li>
    /// <li>
    /// <p><b>Disassociated</b>: CodeGuru Reviewer successfully disassociated the repository. You can create a new association with this repository if you want to review source code in it later. You can control access to code reviews created in anassociated repository with tags after it has been disassociated. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/auth-and-access-control-using-tags.html">Using tags to control access to associated repositories</a> in the <i>Amazon CodeGuru Reviewer User Guide</i>.</p></li>
    /// </ul>
    pub fn states(mut self, input: crate::types::RepositoryAssociationState) -> Self {
        self.inner = self.inner.states(input);
        self
    }
    /// <p>List of repository association states to use as a filter.</p>
    /// <p>The valid repository association states are:</p>
    /// <ul>
    /// <li>
    /// <p><b>Associated</b>: The repository association is complete.</p></li>
    /// <li>
    /// <p><b>Associating</b>: CodeGuru Reviewer is:</p>
    /// <ul>
    /// <li>
    /// <p>Setting up pull request notifications. This is required for pull requests to trigger a CodeGuru Reviewer review.</p><note>
    /// <p>If your repository <code>ProviderType</code> is <code>GitHub</code>, <code>GitHub Enterprise Server</code>, or <code>Bitbucket</code>, CodeGuru Reviewer creates webhooks in your repository to trigger CodeGuru Reviewer reviews. If you delete these webhooks, reviews of code in your repository cannot be triggered.</p>
    /// </note></li>
    /// <li>
    /// <p>Setting up source code access. This is required for CodeGuru Reviewer to securely clone code in your repository.</p></li>
    /// </ul></li>
    /// <li>
    /// <p><b>Failed</b>: The repository failed to associate or disassociate.</p></li>
    /// <li>
    /// <p><b>Disassociating</b>: CodeGuru Reviewer is removing the repository's pull request notifications and source code access.</p></li>
    /// <li>
    /// <p><b>Disassociated</b>: CodeGuru Reviewer successfully disassociated the repository. You can create a new association with this repository if you want to review source code in it later. You can control access to code reviews created in anassociated repository with tags after it has been disassociated. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/auth-and-access-control-using-tags.html">Using tags to control access to associated repositories</a> in the <i>Amazon CodeGuru Reviewer User Guide</i>.</p></li>
    /// </ul>
    pub fn set_states(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RepositoryAssociationState>>) -> Self {
        self.inner = self.inner.set_states(input);
        self
    }
    /// <p>List of repository association states to use as a filter.</p>
    /// <p>The valid repository association states are:</p>
    /// <ul>
    /// <li>
    /// <p><b>Associated</b>: The repository association is complete.</p></li>
    /// <li>
    /// <p><b>Associating</b>: CodeGuru Reviewer is:</p>
    /// <ul>
    /// <li>
    /// <p>Setting up pull request notifications. This is required for pull requests to trigger a CodeGuru Reviewer review.</p><note>
    /// <p>If your repository <code>ProviderType</code> is <code>GitHub</code>, <code>GitHub Enterprise Server</code>, or <code>Bitbucket</code>, CodeGuru Reviewer creates webhooks in your repository to trigger CodeGuru Reviewer reviews. If you delete these webhooks, reviews of code in your repository cannot be triggered.</p>
    /// </note></li>
    /// <li>
    /// <p>Setting up source code access. This is required for CodeGuru Reviewer to securely clone code in your repository.</p></li>
    /// </ul></li>
    /// <li>
    /// <p><b>Failed</b>: The repository failed to associate or disassociate.</p></li>
    /// <li>
    /// <p><b>Disassociating</b>: CodeGuru Reviewer is removing the repository's pull request notifications and source code access.</p></li>
    /// <li>
    /// <p><b>Disassociated</b>: CodeGuru Reviewer successfully disassociated the repository. You can create a new association with this repository if you want to review source code in it later. You can control access to code reviews created in anassociated repository with tags after it has been disassociated. For more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/auth-and-access-control-using-tags.html">Using tags to control access to associated repositories</a> in the <i>Amazon CodeGuru Reviewer User Guide</i>.</p></li>
    /// </ul>
    pub fn get_states(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RepositoryAssociationState>> {
        self.inner.get_states()
    }
    /// Appends an item to `Names`.
    ///
    /// To override the contents of this collection use [`set_names`](Self::set_names).
    ///
    /// <p>List of repository names to use as a filter.</p>
    pub fn names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.names(input.into());
        self
    }
    /// <p>List of repository names to use as a filter.</p>
    pub fn set_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_names(input);
        self
    }
    /// <p>List of repository names to use as a filter.</p>
    pub fn get_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_names()
    }
    /// Appends an item to `Owners`.
    ///
    /// To override the contents of this collection use [`set_owners`](Self::set_owners).
    ///
    /// <p>List of owners to use as a filter. For Amazon Web Services CodeCommit, it is the name of the CodeCommit account that was used to associate the repository. For other repository source providers, such as Bitbucket and GitHub Enterprise Server, this is name of the account that was used to associate the repository.</p>
    pub fn owners(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.owners(input.into());
        self
    }
    /// <p>List of owners to use as a filter. For Amazon Web Services CodeCommit, it is the name of the CodeCommit account that was used to associate the repository. For other repository source providers, such as Bitbucket and GitHub Enterprise Server, this is name of the account that was used to associate the repository.</p>
    pub fn set_owners(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_owners(input);
        self
    }
    /// <p>List of owners to use as a filter. For Amazon Web Services CodeCommit, it is the name of the CodeCommit account that was used to associate the repository. For other repository source providers, such as Bitbucket and GitHub Enterprise Server, this is name of the account that was used to associate the repository.</p>
    pub fn get_owners(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_owners()
    }
    /// <p>The maximum number of repository association results returned by <code>ListRepositoryAssociations</code> in paginated output. When this parameter is used, <code>ListRepositoryAssociations</code> only returns <code>maxResults</code> results in a single page with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListRepositoryAssociations</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, <code>ListRepositoryAssociations</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of repository association results returned by <code>ListRepositoryAssociations</code> in paginated output. When this parameter is used, <code>ListRepositoryAssociations</code> only returns <code>maxResults</code> results in a single page with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListRepositoryAssociations</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, <code>ListRepositoryAssociations</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of repository association results returned by <code>ListRepositoryAssociations</code> in paginated output. When this parameter is used, <code>ListRepositoryAssociations</code> only returns <code>maxResults</code> results in a single page with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListRepositoryAssociations</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, <code>ListRepositoryAssociations</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListRepositoryAssociations</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p><note>
    /// <p>Treat this token as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListRepositoryAssociations</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p><note>
    /// <p>Treat this token as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListRepositoryAssociations</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p><note>
    /// <p>Treat this token as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
