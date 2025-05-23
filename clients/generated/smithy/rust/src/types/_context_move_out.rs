// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContextMoveOut  {
    #[allow(missing_docs)] // documentation missing in model
    pub context_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub override_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub weight: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub description: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub change_reason: ::std::option::Option<::std::string::String>,
}
impl  ContextMoveOut  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn context_id(&self) -> ::std::option::Option<&str> {
        self.context_id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn override_id(&self) -> ::std::option::Option<&str> {
        self.override_id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn weight(&self) -> ::std::option::Option<&str> {
        self.weight.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn change_reason(&self) -> ::std::option::Option<&str> {
        self.change_reason.as_deref()
    }
}
impl ContextMoveOut {
    /// Creates a new builder-style object to manufacture [`ContextMoveOut`](crate::types::ContextMoveOut).
    pub fn builder() -> crate::types::builders::ContextMoveOutBuilder {
        crate::types::builders::ContextMoveOutBuilder::default()
    }
}

/// A builder for [`ContextMoveOut`](crate::types::ContextMoveOut).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ContextMoveOutBuilder {
    pub(crate) context_id: ::std::option::Option<::std::string::String>,
    pub(crate) override_id: ::std::option::Option<::std::string::String>,
    pub(crate) weight: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) change_reason: ::std::option::Option<::std::string::String>,
}
impl ContextMoveOutBuilder {
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
    pub fn weight(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.weight = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_weight(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.weight = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_weight(&self) -> &::std::option::Option<::std::string::String> {
        &self.weight
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn change_reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.change_reason = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_change_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.change_reason = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_change_reason(&self) -> &::std::option::Option<::std::string::String> {
        &self.change_reason
    }
    /// Consumes the builder and constructs a [`ContextMoveOut`](crate::types::ContextMoveOut).
    pub fn build(self) -> crate::types::ContextMoveOut {
        crate::types::ContextMoveOut {
            context_id: self.context_id
            ,
            override_id: self.override_id
            ,
            weight: self.weight
            ,
            description: self.description
            ,
            change_reason: self.change_reason
            ,
        }
    }
}

