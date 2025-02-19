// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_nat_gateway_address::_associate_nat_gateway_address_output::AssociateNatGatewayAddressOutputBuilder;

pub use crate::operation::associate_nat_gateway_address::_associate_nat_gateway_address_input::AssociateNatGatewayAddressInputBuilder;

impl AssociateNatGatewayAddressInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_nat_gateway_address();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateNatGatewayAddress`.
///
/// <p>Associates Elastic IP addresses (EIPs) and private IPv4 addresses with a public NAT gateway. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html#nat-gateway-working-with">Work with NAT gateways</a> in the <i>Amazon VPC User Guide</i>.</p>
/// <p>By default, you can associate up to 2 Elastic IP addresses per public NAT gateway. You can increase the limit by requesting a quota adjustment. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/amazon-vpc-limits.html#vpc-limits-eips">Elastic IP address quotas</a> in the <i>Amazon VPC User Guide</i>.</p><important>
/// <p>When you associate an EIP or secondary EIPs with a public NAT gateway, the network border group of the EIPs must match the network border group of the Availability Zone (AZ) that the public NAT gateway is in. If it's not the same, the EIP will fail to associate. You can see the network border group for the subnet's AZ by viewing the details of the subnet. Similarly, you can view the network border group of an EIP by viewing the details of the EIP address. For more information about network border groups and EIPs, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-eips.html#allocate-eip">Allocate an Elastic IP address</a> in the <i>Amazon VPC User Guide</i>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateNatGatewayAddressFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_nat_gateway_address::builders::AssociateNatGatewayAddressInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput,
        crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError,
    > for AssociateNatGatewayAddressFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput,
            crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateNatGatewayAddressFluentBuilder {
    /// Creates a new `AssociateNatGatewayAddress`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateNatGatewayAddress as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_nat_gateway_address::builders::AssociateNatGatewayAddressInputBuilder {
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
        crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddress::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddress::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput,
        crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError,
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
    /// <p>The ID of the NAT gateway.</p>
    pub fn nat_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.nat_gateway_id(input.into());
        self
    }
    /// <p>The ID of the NAT gateway.</p>
    pub fn set_nat_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_nat_gateway_id(input);
        self
    }
    /// <p>The ID of the NAT gateway.</p>
    pub fn get_nat_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_nat_gateway_id()
    }
    /// Appends an item to `AllocationIds`.
    ///
    /// To override the contents of this collection use [`set_allocation_ids`](Self::set_allocation_ids).
    ///
    /// <p>The allocation IDs of EIPs that you want to associate with your NAT gateway.</p>
    pub fn allocation_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.allocation_ids(input.into());
        self
    }
    /// <p>The allocation IDs of EIPs that you want to associate with your NAT gateway.</p>
    pub fn set_allocation_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_allocation_ids(input);
        self
    }
    /// <p>The allocation IDs of EIPs that you want to associate with your NAT gateway.</p>
    pub fn get_allocation_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_allocation_ids()
    }
    /// Appends an item to `PrivateIpAddresses`.
    ///
    /// To override the contents of this collection use [`set_private_ip_addresses`](Self::set_private_ip_addresses).
    ///
    /// <p>The private IPv4 addresses that you want to assign to the NAT gateway.</p>
    pub fn private_ip_addresses(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.private_ip_addresses(input.into());
        self
    }
    /// <p>The private IPv4 addresses that you want to assign to the NAT gateway.</p>
    pub fn set_private_ip_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_private_ip_addresses(input);
        self
    }
    /// <p>The private IPv4 addresses that you want to assign to the NAT gateway.</p>
    pub fn get_private_ip_addresses(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_private_ip_addresses()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
