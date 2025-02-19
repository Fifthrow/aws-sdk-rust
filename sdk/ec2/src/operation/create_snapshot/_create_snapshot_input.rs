// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateSnapshotInput {
    /// <p>A description for the snapshot.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.</p>
    /// <ul>
    /// <li>
    /// <p>To create a snapshot of a volume in a Region, omit this parameter. The snapshot is created in the same Region as the volume.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot in the Region, omit this parameter. The snapshot is created in the Region for the Outpost.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot on an Outpost, specify the ARN of the destination Outpost. The snapshot must be created on the same Outpost as the volume.</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-snapshot">Create local snapshots from volumes on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub outpost_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon EBS volume.</p>
    pub volume_id: ::std::option::Option<::std::string::String>,
    /// <p>The tags to apply to the snapshot during creation.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl CreateSnapshotInput {
    /// <p>A description for the snapshot.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.</p>
    /// <ul>
    /// <li>
    /// <p>To create a snapshot of a volume in a Region, omit this parameter. The snapshot is created in the same Region as the volume.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot in the Region, omit this parameter. The snapshot is created in the Region for the Outpost.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot on an Outpost, specify the ARN of the destination Outpost. The snapshot must be created on the same Outpost as the volume.</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-snapshot">Create local snapshots from volumes on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn outpost_arn(&self) -> ::std::option::Option<&str> {
        self.outpost_arn.as_deref()
    }
    /// <p>The ID of the Amazon EBS volume.</p>
    pub fn volume_id(&self) -> ::std::option::Option<&str> {
        self.volume_id.as_deref()
    }
    /// <p>The tags to apply to the snapshot during creation.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl CreateSnapshotInput {
    /// Creates a new builder-style object to manufacture [`CreateSnapshotInput`](crate::operation::create_snapshot::CreateSnapshotInput).
    pub fn builder() -> crate::operation::create_snapshot::builders::CreateSnapshotInputBuilder {
        crate::operation::create_snapshot::builders::CreateSnapshotInputBuilder::default()
    }
}

/// A builder for [`CreateSnapshotInput`](crate::operation::create_snapshot::CreateSnapshotInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateSnapshotInputBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) outpost_arn: ::std::option::Option<::std::string::String>,
    pub(crate) volume_id: ::std::option::Option<::std::string::String>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl CreateSnapshotInputBuilder {
    /// <p>A description for the snapshot.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the snapshot.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description for the snapshot.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.</p>
    /// <ul>
    /// <li>
    /// <p>To create a snapshot of a volume in a Region, omit this parameter. The snapshot is created in the same Region as the volume.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot in the Region, omit this parameter. The snapshot is created in the Region for the Outpost.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot on an Outpost, specify the ARN of the destination Outpost. The snapshot must be created on the same Outpost as the volume.</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-snapshot">Create local snapshots from volumes on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn outpost_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.outpost_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.</p>
    /// <ul>
    /// <li>
    /// <p>To create a snapshot of a volume in a Region, omit this parameter. The snapshot is created in the same Region as the volume.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot in the Region, omit this parameter. The snapshot is created in the Region for the Outpost.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot on an Outpost, specify the ARN of the destination Outpost. The snapshot must be created on the same Outpost as the volume.</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-snapshot">Create local snapshots from volumes on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn set_outpost_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.outpost_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.</p>
    /// <ul>
    /// <li>
    /// <p>To create a snapshot of a volume in a Region, omit this parameter. The snapshot is created in the same Region as the volume.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot in the Region, omit this parameter. The snapshot is created in the Region for the Outpost.</p></li>
    /// <li>
    /// <p>To create a snapshot of a volume on an Outpost and store the snapshot on an Outpost, specify the ARN of the destination Outpost. The snapshot must be created on the same Outpost as the volume.</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-snapshot">Create local snapshots from volumes on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn get_outpost_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.outpost_arn
    }
    /// <p>The ID of the Amazon EBS volume.</p>
    /// This field is required.
    pub fn volume_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.volume_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon EBS volume.</p>
    pub fn set_volume_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.volume_id = input;
        self
    }
    /// <p>The ID of the Amazon EBS volume.</p>
    pub fn get_volume_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.volume_id
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the snapshot during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the snapshot during creation.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to apply to the snapshot during creation.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`CreateSnapshotInput`](crate::operation::create_snapshot::CreateSnapshotInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_snapshot::CreateSnapshotInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_snapshot::CreateSnapshotInput {
            description: self.description,
            outpost_arn: self.outpost_arn,
            volume_id: self.volume_id,
            tag_specifications: self.tag_specifications,
            dry_run: self.dry_run,
        })
    }
}
