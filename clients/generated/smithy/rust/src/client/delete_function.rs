// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteFunction`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::set_workspace_id):<br>required: **true**<br>(undocumented)<br>
    ///   - [`org_id(impl Into<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::org_id) / [`set_org_id(Option<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::set_org_id):<br>required: **true**<br>(undocumented)<br>
    ///   - [`function_name(impl Into<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::set_function_name):<br>required: **true**<br>(undocumented)<br>
                            /// - On success, responds with [`DeleteFunctionOutput`](crate::operation::delete_function::DeleteFunctionOutput)
                            /// - On failure, responds with [`SdkError<DeleteFunctionError>`](crate::operation::delete_function::DeleteFunctionError)
    pub fn delete_function(&self) -> crate::operation::delete_function::builders::DeleteFunctionFluentBuilder {
                                crate::operation::delete_function::builders::DeleteFunctionFluentBuilder::new(self.handle.clone())
                            }
}

