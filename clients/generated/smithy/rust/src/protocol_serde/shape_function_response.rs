// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_function_response<'a, I>(tokens: &mut ::std::iter::Peekable<I>) -> ::std::result::Result<Option<crate::types::FunctionResponse>, ::aws_smithy_json::deserialize::error::DeserializeError>
                        where I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::FunctionResponseBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "function_name" => {
                                builder = builder.set_function_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "published_code" => {
                                builder = builder.set_published_code(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "draft_code" => {
                                builder = builder.set_draft_code(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "published_runtime_version" => {
                                builder = builder.set_published_runtime_version(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "draft_runtime_version" => {
                                builder = builder.set_draft_runtime_version(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "published_at" => {
                                builder = builder.set_published_at(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::DateTimeWithOffset)?
                                );
                            }
                            "draft_edited_at" => {
                                builder = builder.set_draft_edited_at(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::DateTimeWithOffset)?
                                );
                            }
                            "published_by" => {
                                builder = builder.set_published_by(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "draft_edited_by" => {
                                builder = builder.set_draft_edited_by(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "last_modified_at" => {
                                builder = builder.set_last_modified_at(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), ::aws_smithy_types::date_time::Format::DateTimeWithOffset)?
                                );
                            }
                            "last_modified_by" => {
                                builder = builder.set_last_modified_by(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "change_reason" => {
                                builder = builder.set_change_reason(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "description" => {
                                builder = builder.set_description(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "function_type" => {
                                builder = builder.set_function_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::types::FunctionTypes::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(crate::serde_util::function_response_correct_errors(builder).build().map_err(|err|::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?))
        }
        _ => {
            Err(::aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

