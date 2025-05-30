// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Variant  {
    #[allow(missing_docs)] // documentation missing in model
    pub id: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub variant_type: crate::types::VariantType,
    #[allow(missing_docs)] // documentation missing in model
    pub context_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub override_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub overrides: ::aws_smithy_types::Document,
}
impl  Variant  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn id(&self) -> &str {
        use std::ops::Deref; self.id.deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn variant_type(&self) -> &crate::types::VariantType {
        &self.variant_type
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn context_id(&self) -> ::std::option::Option<&str> {
        self.context_id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn override_id(&self) -> ::std::option::Option<&str> {
        self.override_id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn overrides(&self) -> &::aws_smithy_types::Document {
        &self.overrides
    }
}
impl Variant {
    /// Creates a new builder-style object to manufacture [`Variant`](crate::types::Variant).
    pub fn builder() -> crate::types::builders::VariantBuilder {
        crate::types::builders::VariantBuilder::default()
    }
}

/// A builder for [`Variant`](crate::types::Variant).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct VariantBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) variant_type: ::std::option::Option<crate::types::VariantType>,
    pub(crate) context_id: ::std::option::Option<::std::string::String>,
    pub(crate) override_id: ::std::option::Option<::std::string::String>,
    pub(crate) overrides: ::std::option::Option<::aws_smithy_types::Document>,
}
impl VariantBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn variant_type(mut self, input: crate::types::VariantType) -> Self {
        self.variant_type = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_variant_type(mut self, input: ::std::option::Option<crate::types::VariantType>) -> Self {
        self.variant_type = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_variant_type(&self) -> &::std::option::Option<crate::types::VariantType> {
        &self.variant_type
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn context_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.context_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_context_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.context_id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_context_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.context_id
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn override_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.override_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_override_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.override_id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_override_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.override_id
    }
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn overrides(mut self, input: ::aws_smithy_types::Document) -> Self {
        self.overrides = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_overrides(mut self, input: ::std::option::Option<::aws_smithy_types::Document>) -> Self {
        self.overrides = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_overrides(&self) -> &::std::option::Option<::aws_smithy_types::Document> {
        &self.overrides
    }
    /// Consumes the builder and constructs a [`Variant`](crate::types::Variant).
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](crate::types::builders::VariantBuilder::id)
    /// - [`variant_type`](crate::types::builders::VariantBuilder::variant_type)
    /// - [`overrides`](crate::types::builders::VariantBuilder::overrides)
    pub fn build(self) -> ::std::result::Result<crate::types::Variant, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::types::Variant {
                id: self.id
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("id", "id was not specified but it is required when building Variant")
                    )?
                ,
                variant_type: self.variant_type
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("variant_type", "variant_type was not specified but it is required when building Variant")
                    )?
                ,
                context_id: self.context_id
                ,
                override_id: self.override_id
                ,
                overrides: self.overrides
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("overrides", "overrides was not specified but it is required when building Variant")
                    )?
                ,
            }
        )
    }
}

