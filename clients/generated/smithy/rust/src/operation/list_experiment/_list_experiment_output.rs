// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListExperimentOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub total_pages: i64,
    #[allow(missing_docs)] // documentation missing in model
    pub total_items: i64,
    #[allow(missing_docs)] // documentation missing in model
    pub data: ::std::vec::Vec::<crate::types::ExperimentResponse>,
}
impl  ListExperimentOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn total_pages(&self) -> i64 {
        self.total_pages
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn total_items(&self) -> i64 {
        self.total_items
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn data(&self) -> &[crate::types::ExperimentResponse] {
        use std::ops::Deref; self.data.deref()
    }
}
impl ListExperimentOutput {
    /// Creates a new builder-style object to manufacture [`ListExperimentOutput`](crate::operation::list_experiment::ListExperimentOutput).
    pub fn builder() -> crate::operation::list_experiment::builders::ListExperimentOutputBuilder {
        crate::operation::list_experiment::builders::ListExperimentOutputBuilder::default()
    }
}

/// A builder for [`ListExperimentOutput`](crate::operation::list_experiment::ListExperimentOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListExperimentOutputBuilder {
    pub(crate) total_pages: ::std::option::Option<i64>,
    pub(crate) total_items: ::std::option::Option<i64>,
    pub(crate) data: ::std::option::Option<::std::vec::Vec::<crate::types::ExperimentResponse>>,
}
impl ListExperimentOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn total_pages(mut self, input: i64) -> Self {
        self.total_pages = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_total_pages(mut self, input: ::std::option::Option<i64>) -> Self {
        self.total_pages = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_total_pages(&self) -> &::std::option::Option<i64> {
        &self.total_pages
    }
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn total_items(mut self, input: i64) -> Self {
        self.total_items = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_total_items(mut self, input: ::std::option::Option<i64>) -> Self {
        self.total_items = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_total_items(&self) -> &::std::option::Option<i64> {
        &self.total_items
    }
    /// Appends an item to `data`.
    ///
    /// To override the contents of this collection use [`set_data`](Self::set_data).
    ///
    pub fn data(mut self, input: crate::types::ExperimentResponse) -> Self {
        let mut v = self.data.unwrap_or_default();
                        v.push(input);
                        self.data = ::std::option::Option::Some(v);
                        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_data(mut self, input: ::std::option::Option<::std::vec::Vec::<crate::types::ExperimentResponse>>) -> Self {
        self.data = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_data(&self) -> &::std::option::Option<::std::vec::Vec::<crate::types::ExperimentResponse>> {
        &self.data
    }
    /// Consumes the builder and constructs a [`ListExperimentOutput`](crate::operation::list_experiment::ListExperimentOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`total_pages`](crate::operation::list_experiment::builders::ListExperimentOutputBuilder::total_pages)
    /// - [`total_items`](crate::operation::list_experiment::builders::ListExperimentOutputBuilder::total_items)
    /// - [`data`](crate::operation::list_experiment::builders::ListExperimentOutputBuilder::data)
    pub fn build(self) -> ::std::result::Result<crate::operation::list_experiment::ListExperimentOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::list_experiment::ListExperimentOutput {
                total_pages: self.total_pages
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("total_pages", "total_pages was not specified but it is required when building ListExperimentOutput")
                    )?
                ,
                total_items: self.total_items
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("total_items", "total_items was not specified but it is required when building ListExperimentOutput")
                    )?
                ,
                data: self.data
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("data", "data was not specified but it is required when building ListExperimentOutput")
                    )?
                ,
            }
        )
    }
}

