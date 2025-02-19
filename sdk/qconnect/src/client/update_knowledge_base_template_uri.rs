// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateKnowledgeBaseTemplateUri`](crate::operation::update_knowledge_base_template_uri::builders::UpdateKnowledgeBaseTemplateUriFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`knowledge_base_id(impl Into<String>)`](crate::operation::update_knowledge_base_template_uri::builders::UpdateKnowledgeBaseTemplateUriFluentBuilder::knowledge_base_id) / [`set_knowledge_base_id(Option<String>)`](crate::operation::update_knowledge_base_template_uri::builders::UpdateKnowledgeBaseTemplateUriFluentBuilder::set_knowledge_base_id):<br>required: **true**<br><p>The identifier of the knowledge base. This should not be a QUICK_RESPONSES type knowledge base if you're storing Amazon Q Content resource to it. Can be either the ID or the ARN. URLs cannot contain the ARN.</p><br>
    ///   - [`template_uri(impl Into<String>)`](crate::operation::update_knowledge_base_template_uri::builders::UpdateKnowledgeBaseTemplateUriFluentBuilder::template_uri) / [`set_template_uri(Option<String>)`](crate::operation::update_knowledge_base_template_uri::builders::UpdateKnowledgeBaseTemplateUriFluentBuilder::set_template_uri):<br>required: **true**<br><p>The template URI to update.</p><br>
    /// - On success, responds with [`UpdateKnowledgeBaseTemplateUriOutput`](crate::operation::update_knowledge_base_template_uri::UpdateKnowledgeBaseTemplateUriOutput) with field(s):
    ///   - [`knowledge_base(Option<KnowledgeBaseData>)`](crate::operation::update_knowledge_base_template_uri::UpdateKnowledgeBaseTemplateUriOutput::knowledge_base): <p>The knowledge base to update.</p>
    /// - On failure, responds with [`SdkError<UpdateKnowledgeBaseTemplateUriError>`](crate::operation::update_knowledge_base_template_uri::UpdateKnowledgeBaseTemplateUriError)
    pub fn update_knowledge_base_template_uri(
        &self,
    ) -> crate::operation::update_knowledge_base_template_uri::builders::UpdateKnowledgeBaseTemplateUriFluentBuilder {
        crate::operation::update_knowledge_base_template_uri::builders::UpdateKnowledgeBaseTemplateUriFluentBuilder::new(self.handle.clone())
    }
}
