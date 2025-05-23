// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BulkOperationOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bulk_operation_output: ::std::option::Option<crate::types::BulkOperationOut>,
}
impl  BulkOperationOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bulk_operation_output(&self) -> ::std::option::Option<&crate::types::BulkOperationOut> {
        self.bulk_operation_output.as_ref()
    }
}
impl BulkOperationOutput {
    /// Creates a new builder-style object to manufacture [`BulkOperationOutput`](crate::operation::bulk_operation::BulkOperationOutput).
    pub fn builder() -> crate::operation::bulk_operation::builders::BulkOperationOutputBuilder {
        crate::operation::bulk_operation::builders::BulkOperationOutputBuilder::default()
    }
}

/// A builder for [`BulkOperationOutput`](crate::operation::bulk_operation::BulkOperationOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct BulkOperationOutputBuilder {
    pub(crate) bulk_operation_output: ::std::option::Option<crate::types::BulkOperationOut>,
}
impl BulkOperationOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bulk_operation_output(mut self, input: crate::types::BulkOperationOut) -> Self {
        self.bulk_operation_output = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_bulk_operation_output(mut self, input: ::std::option::Option<crate::types::BulkOperationOut>) -> Self {
        self.bulk_operation_output = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_bulk_operation_output(&self) -> &::std::option::Option<crate::types::BulkOperationOut> {
        &self.bulk_operation_output
    }
    /// Consumes the builder and constructs a [`BulkOperationOutput`](crate::operation::bulk_operation::BulkOperationOutput).
    pub fn build(self) -> crate::operation::bulk_operation::BulkOperationOutput {
        crate::operation::bulk_operation::BulkOperationOutput {
            bulk_operation_output: self.bulk_operation_output
            ,
        }
    }
}

