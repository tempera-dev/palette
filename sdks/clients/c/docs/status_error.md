# status_error_t

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** | The HTTP status code, repeated in the body so the envelope is self-describing. |
**details** | **list_t \*** | Structured error details. Present but empty when there are none. | [optional]
**message** | **char \*** | A developer-facing description that is safe to log and safe to show. |
**request_id** | **char \*** | The server-assigned request identifier, mirrored from the x-request-id response header. | [optional]
**status** | **palette_api_status_error_STATUS_e** | The canonical google.rpc.Code name, for example INVALID_ARGUMENT. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
