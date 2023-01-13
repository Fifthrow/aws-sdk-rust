// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Temporary shim to allow new and old endpoint resolvers to co-exist
///
/// This enables converting from the actual parameters type to the placeholder parameters type that
/// contains a region
#[doc(hidden)]
impl From<crate::endpoint::Params> for aws_endpoint::Params {
    fn from(params: crate::endpoint::Params) -> Self {
        Self::new(
            params
                .region()
                .map(|r| aws_types::region::Region::new(r.to_string())),
        )
    }
}

/// Generated endpoint tests
#[cfg(test)]
mod test {

    /// For region aws-global with FIPS disabled and DualStack disabled
    #[test]
    fn test_1() {
        use aws_smithy_http::endpoint::ResolveEndpoint;
        let params = crate::endpoint::Params::builder()
            .region("aws-global".to_string())
            .use_dual_stack(false)
            .use_fips(false)
            .build()
            .expect("invalid params");
        let resolver = crate::endpoint::DefaultResolver::new();
        let endpoint = resolver.resolve_endpoint(&params);
        let endpoint =
            endpoint.expect("Expected valid endpoint: https://savingsplans.amazonaws.com");
        assert_eq!(
            endpoint,
            aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://savingsplans.amazonaws.com")
                .property(
                    "authSchemes",
                    vec![aws_smithy_types::Document::from({
                        let mut out =
                            std::collections::HashMap::<String, aws_smithy_types::Document>::new();
                        out.insert("name".to_string(), "sigv4".to_string().into());
                        out.insert("signingRegion".to_string(), "us-east-1".to_string().into());
                        out.insert("signingName".to_string(), "savingsplans".to_string().into());
                        out
                    })]
                )
                .build()
        );
    }

    /// For custom endpoint with fips disabled and dualstack disabled
    #[test]
    fn test_2() {
        use aws_smithy_http::endpoint::ResolveEndpoint;
        let params = crate::endpoint::Params::builder()
            .region("us-east-1".to_string())
            .use_dual_stack(false)
            .use_fips(false)
            .endpoint("https://example.com".to_string())
            .build()
            .expect("invalid params");
        let resolver = crate::endpoint::DefaultResolver::new();
        let endpoint = resolver.resolve_endpoint(&params);
        let endpoint = endpoint.expect("Expected valid endpoint: https://example.com");
        assert_eq!(
            endpoint,
            aws_smithy_types::endpoint::Endpoint::builder()
                .url("https://example.com")
                .build()
        );
    }

    /// For custom endpoint with fips enabled and dualstack disabled
    #[test]
    fn test_3() {
        use aws_smithy_http::endpoint::ResolveEndpoint;
        let params = crate::endpoint::Params::builder()
            .region("us-east-1".to_string())
            .use_dual_stack(false)
            .use_fips(true)
            .endpoint("https://example.com".to_string())
            .build()
            .expect("invalid params");
        let resolver = crate::endpoint::DefaultResolver::new();
        let endpoint = resolver.resolve_endpoint(&params);
        let error = endpoint.expect_err("expected error: Invalid Configuration: FIPS and custom endpoint are not supported [For custom endpoint with fips enabled and dualstack disabled]");
        assert_eq!(
            format!("{}", error),
            "Invalid Configuration: FIPS and custom endpoint are not supported"
        )
    }

    /// For custom endpoint with fips disabled and dualstack enabled
    #[test]
    fn test_4() {
        use aws_smithy_http::endpoint::ResolveEndpoint;
        let params = crate::endpoint::Params::builder()
            .region("us-east-1".to_string())
            .use_dual_stack(true)
            .use_fips(false)
            .endpoint("https://example.com".to_string())
            .build()
            .expect("invalid params");
        let resolver = crate::endpoint::DefaultResolver::new();
        let endpoint = resolver.resolve_endpoint(&params);
        let error = endpoint.expect_err("expected error: Invalid Configuration: Dualstack and custom endpoint are not supported [For custom endpoint with fips disabled and dualstack enabled]");
        assert_eq!(
            format!("{}", error),
            "Invalid Configuration: Dualstack and custom endpoint are not supported"
        )
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
/// Configuration parameters for resolving the correct endpoint
pub struct Params {
    /// The AWS region used to dispatch the request.
    pub(crate) region: std::string::String,
    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
    pub(crate) use_dual_stack: bool,
    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
    pub(crate) use_fips: bool,
    /// Override the endpoint used to send this request
    pub(crate) endpoint: std::option::Option<std::string::String>,
}
impl Params {
    /// Create a builder for [`Params`]
    pub fn builder() -> crate::endpoint::ParamsBuilder {
        crate::endpoint::ParamsBuilder::default()
    }
    /// The AWS region used to dispatch the request.
    pub fn region(&self) -> std::option::Option<&str> {
        Some(&self.region)
    }
    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
    pub fn use_dual_stack(&self) -> std::option::Option<bool> {
        Some(self.use_dual_stack)
    }
    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
    pub fn use_fips(&self) -> std::option::Option<bool> {
        Some(self.use_fips)
    }
    /// Override the endpoint used to send this request
    pub fn endpoint(&self) -> std::option::Option<&str> {
        self.endpoint.as_deref()
    }
}

/// The default endpoint resolver
#[derive(Default)]
pub struct DefaultResolver {
    partition_resolver: crate::endpoint_lib::partition::PartitionResolver,
}

impl DefaultResolver {
    /// Create a new endpoint resolver with default settings
    pub fn new() -> Self {
        Self { partition_resolver: crate::endpoint_lib::partition::PartitionResolver::new_from_json(b"{\"version\":\"1.1\",\"partitions\":[{\"id\":\"aws\",\"regionRegex\":\"^(us|eu|ap|sa|ca|me|af)-\\\\w+-\\\\d+$\",\"regions\":{\"af-south-1\":{},\"ap-east-1\":{},\"ap-northeast-1\":{},\"ap-northeast-2\":{},\"ap-northeast-3\":{},\"ap-south-1\":{},\"ap-southeast-1\":{},\"ap-southeast-2\":{},\"ap-southeast-3\":{},\"ca-central-1\":{},\"eu-central-1\":{},\"eu-north-1\":{},\"eu-south-1\":{},\"eu-west-1\":{},\"eu-west-2\":{},\"eu-west-3\":{},\"me-central-1\":{},\"me-south-1\":{},\"sa-east-1\":{},\"us-east-1\":{},\"us-east-2\":{},\"us-west-1\":{},\"us-west-2\":{},\"aws-global\":{}},\"outputs\":{\"name\":\"aws\",\"dnsSuffix\":\"amazonaws.com\",\"dualStackDnsSuffix\":\"api.aws\",\"supportsFIPS\":true,\"supportsDualStack\":true}},{\"id\":\"aws-us-gov\",\"regionRegex\":\"^us\\\\-gov\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"us-gov-west-1\":{},\"us-gov-east-1\":{},\"aws-us-gov-global\":{}},\"outputs\":{\"name\":\"aws-us-gov\",\"dnsSuffix\":\"amazonaws.com\",\"dualStackDnsSuffix\":\"api.aws\",\"supportsFIPS\":true,\"supportsDualStack\":true}},{\"id\":\"aws-cn\",\"regionRegex\":\"^cn\\\\-\\\\w+\\\\-\\\\d+$\",\"regions\":{\"cn-north-1\":{},\"cn-northwest-1\":{},\"aws-cn-global\":{}},\"outputs\":{\"name\":\"aws-cn\",\"dnsSuffix\":\"amazonaws.com.cn\",\"dualStackDnsSuffix\":\"api.amazonwebservices.com.cn\",\"supportsFIPS\":true,\"supportsDualStack\":true}},{\"id\":\"aws-iso\",\"regionRegex\":\"^us\\\\-iso\\\\-\\\\w+\\\\-\\\\d+$\",\"outputs\":{\"name\":\"aws-iso\",\"dnsSuffix\":\"c2s.ic.gov\",\"supportsFIPS\":true,\"supportsDualStack\":false,\"dualStackDnsSuffix\":\"c2s.ic.gov\"},\"regions\":{\"us-iso-east-1\":{},\"us-iso-west-1\":{},\"aws-iso-global\":{}}},{\"id\":\"aws-iso-b\",\"regionRegex\":\"^us\\\\-isob\\\\-\\\\w+\\\\-\\\\d+$\",\"outputs\":{\"name\":\"aws-iso-b\",\"dnsSuffix\":\"sc2s.sgov.gov\",\"supportsFIPS\":true,\"supportsDualStack\":false,\"dualStackDnsSuffix\":\"sc2s.sgov.gov\"},\"regions\":{\"us-isob-east-1\":{},\"aws-iso-b-global\":{}}}]}").expect("valid JSON") }
    }
}

impl aws_smithy_http::endpoint::ResolveEndpoint<crate::endpoint::Params> for DefaultResolver {
    fn resolve_endpoint(&self, params: &Params) -> aws_smithy_http::endpoint::Result {
        let mut diagnostic_collector = crate::endpoint_lib::diagnostic::DiagnosticCollector::new();
        crate::endpoint::internals::resolve_endpoint(
            params,
            &mut diagnostic_collector,
            &self.partition_resolver,
        )
        .map_err(|err| err.with_source(diagnostic_collector.take_last_error()))
    }
}

/// Builder for [`Params`]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ParamsBuilder {
    region: std::option::Option<std::string::String>,
    use_dual_stack: std::option::Option<bool>,
    use_fips: std::option::Option<bool>,
    endpoint: std::option::Option<std::string::String>,
}
impl ParamsBuilder {
    /// Consume this builder, creating [`Params`].
    pub fn build(self) -> Result<crate::endpoint::Params, crate::endpoint::InvalidParams> {
        Ok(crate::endpoint::Params {
            region: self
                .region
                .ok_or_else(|| crate::endpoint::InvalidParams::missing("region"))?,
            use_dual_stack: self
                .use_dual_stack
                .or(Some(false))
                .ok_or_else(|| crate::endpoint::InvalidParams::missing("use_dual_stack"))?,
            use_fips: self
                .use_fips
                .or(Some(false))
                .ok_or_else(|| crate::endpoint::InvalidParams::missing("use_fips"))?,
            endpoint: self.endpoint,
        })
    }
    /// Sets the value for region
    ///
    /// The AWS region used to dispatch the request.
    pub fn region(mut self, value: impl Into<std::string::String>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Sets the value for region
    ///
    /// The AWS region used to dispatch the request.
    pub fn set_region(mut self, param: Option<std::string::String>) -> Self {
        self.region = param;
        self
    }
    /// Sets the value for use_dual_stack
    ///
    /// When unset, this parameter has a default value of `false`.
    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
    pub fn use_dual_stack(mut self, value: impl Into<bool>) -> Self {
        self.use_dual_stack = Some(value.into());
        self
    }

    /// Sets the value for use_dual_stack
    ///
    /// When unset, this parameter has a default value of `false`.
    /// When true, use the dual-stack endpoint. If the configured endpoint does not support dual-stack, dispatching the request MAY return an error.
    pub fn set_use_dual_stack(mut self, param: Option<bool>) -> Self {
        self.use_dual_stack = param;
        self
    }
    /// Sets the value for use_fips
    ///
    /// When unset, this parameter has a default value of `false`.
    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
    pub fn use_fips(mut self, value: impl Into<bool>) -> Self {
        self.use_fips = Some(value.into());
        self
    }

    /// Sets the value for use_fips
    ///
    /// When unset, this parameter has a default value of `false`.
    /// When true, send this request to the FIPS-compliant regional endpoint. If the configured endpoint does not have a FIPS compliant endpoint, dispatching the request will return an error.
    pub fn set_use_fips(mut self, param: Option<bool>) -> Self {
        self.use_fips = param;
        self
    }
    /// Sets the value for endpoint
    ///
    /// Override the endpoint used to send this request
    pub fn endpoint(mut self, value: impl Into<std::string::String>) -> Self {
        self.endpoint = Some(value.into());
        self
    }

    /// Sets the value for endpoint
    ///
    /// Override the endpoint used to send this request
    pub fn set_endpoint(mut self, param: Option<std::string::String>) -> Self {
        self.endpoint = param;
        self
    }
}

/// An error that occurred during endpoint resolution
#[derive(Debug)]
pub struct InvalidParams {
    field: std::borrow::Cow<'static, str>,
}

impl InvalidParams {
    #[allow(dead_code)]
    fn missing(field: &'static str) -> Self {
        Self {
            field: field.into(),
        }
    }
}

impl std::fmt::Display for InvalidParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a required field was missing: `{}`", self.field)
    }
}

impl std::error::Error for InvalidParams {}

/// Endpoints internals
mod internals;
