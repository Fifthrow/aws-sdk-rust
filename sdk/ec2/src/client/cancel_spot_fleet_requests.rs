// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelSpotFleetRequests`](crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`spot_fleet_request_ids(impl Into<String>)`](crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsFluentBuilder::spot_fleet_request_ids) / [`set_spot_fleet_request_ids(Option<Vec::<String>>)`](crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsFluentBuilder::set_spot_fleet_request_ids):<br>required: **true**<br><p>The IDs of the Spot Fleet requests.</p><br>
    ///   - [`terminate_instances(bool)`](crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsFluentBuilder::terminate_instances) / [`set_terminate_instances(Option<bool>)`](crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsFluentBuilder::set_terminate_instances):<br>required: **true**<br><p>Indicates whether to terminate the associated instances when the Spot Fleet request is canceled. The default is to terminate the instances.</p> <p>To let the instances continue to run after the Spot Fleet request is canceled, specify <code>no-terminate-instances</code>.</p><br>
    /// - On success, responds with [`CancelSpotFleetRequestsOutput`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput) with field(s):
    ///   - [`successful_fleet_requests(Option<Vec::<CancelSpotFleetRequestsSuccessItem>>)`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput::successful_fleet_requests): <p>Information about the Spot Fleet requests that are successfully canceled.</p>
    ///   - [`unsuccessful_fleet_requests(Option<Vec::<CancelSpotFleetRequestsErrorItem>>)`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput::unsuccessful_fleet_requests): <p>Information about the Spot Fleet requests that are not successfully canceled.</p>
    /// - On failure, responds with [`SdkError<CancelSpotFleetRequestsError>`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsError)
    pub fn cancel_spot_fleet_requests(&self) -> crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsFluentBuilder {
        crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsFluentBuilder::new(self.handle.clone())
    }
}
