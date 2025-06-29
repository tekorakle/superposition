// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListOrganisationInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub count: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub page: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub all: ::std::option::Option<bool>,
}
impl  ListOrganisationInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn count(&self) -> ::std::option::Option<i32> {
        self.count
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn page(&self) -> ::std::option::Option<i32> {
        self.page
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn all(&self) -> ::std::option::Option<bool> {
        self.all
    }
}
impl ListOrganisationInput {
    /// Creates a new builder-style object to manufacture [`ListOrganisationInput`](crate::operation::list_organisation::ListOrganisationInput).
    pub fn builder() -> crate::operation::list_organisation::builders::ListOrganisationInputBuilder {
        crate::operation::list_organisation::builders::ListOrganisationInputBuilder::default()
    }
}

/// A builder for [`ListOrganisationInput`](crate::operation::list_organisation::ListOrganisationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListOrganisationInputBuilder {
    pub(crate) count: ::std::option::Option<i32>,
    pub(crate) page: ::std::option::Option<i32>,
    pub(crate) all: ::std::option::Option<bool>,
}
impl ListOrganisationInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn count(mut self, input: i32) -> Self {
        self.count = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.count = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_count(&self) -> &::std::option::Option<i32> {
        &self.count
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn page(mut self, input: i32) -> Self {
        self.page = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_page(mut self, input: ::std::option::Option<i32>) -> Self {
        self.page = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_page(&self) -> &::std::option::Option<i32> {
        &self.page
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn all(mut self, input: bool) -> Self {
        self.all = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_all(mut self, input: ::std::option::Option<bool>) -> Self {
        self.all = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_all(&self) -> &::std::option::Option<bool> {
        &self.all
    }
    /// Consumes the builder and constructs a [`ListOrganisationInput`](crate::operation::list_organisation::ListOrganisationInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_organisation::ListOrganisationInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::list_organisation::ListOrganisationInput {
                count: self.count
                ,
                page: self.page
                ,
                all: self.all
                ,
            }
        )
    }
}

