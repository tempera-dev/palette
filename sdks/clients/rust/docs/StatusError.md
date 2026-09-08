# StatusError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **i32** | The HTTP status code, repeated in the body so the envelope is self-describing. |
**details** | Option<[**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md)> | Structured error details. Present but empty when there are none. | [optional]
**message** | **String** | A developer-facing description that is safe to log and safe to show. |
**request_id** | Option<**String**> | The server-assigned request identifier, mirrored from the x-request-id response header. | [optional]
**status** | **String** | The canonical google.rpc.Code name, for example INVALID_ARGUMENT. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
