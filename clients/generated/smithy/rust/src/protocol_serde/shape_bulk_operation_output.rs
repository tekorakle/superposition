// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_bulk_operation_output_payload(body: &[u8]) -> std::result::Result<::std::option::Option<crate::types::BulkOperationOut>, crate::operation::bulk_operation::BulkOperationError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_bulk_operation_out::de_bulk_operation_out_payload(body).map_err(crate::operation::bulk_operation::BulkOperationError::unhandled)
    }).transpose()
}

