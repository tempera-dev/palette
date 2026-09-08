# StatusError


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** | The HTTP status code, repeated in the body so the envelope is self-describing. |
**details** | **List[Dict[str, object]]** | Structured error details. Present but empty when there are none. | [optional]
**message** | **str** | A developer-facing description that is safe to log and safe to show. |
**request_id** | **str** | The server-assigned request identifier, mirrored from the x-request-id response header. | [optional]
**status** | **str** | The canonical google.rpc.Code name, for example INVALID_ARGUMENT. |

## Example

```python
from palette_client.models.status_error import StatusError

# TODO update the JSON string below
json = "{}"
# create an instance of StatusError from a JSON string
status_error_instance = StatusError.from_json(json)
# print the JSON string representation of the object
print(StatusError.to_json())

# convert the object into a dict
status_error_dict = status_error_instance.to_dict()
# create an instance of StatusError from a dict
status_error_from_dict = StatusError.from_dict(status_error_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
