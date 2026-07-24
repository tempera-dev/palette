# ErrorStatus


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** | HTTP status code corresponding to the canonical RPC status. |
**details** | **List[object]** | Machine-readable standard error details. |
**message** | **str** | Developer-facing English problem description. |
**status** | **str** | Canonical &#x60;google.rpc.Code&#x60; enum name. |

## Example

```python
from palette_client.models.error_status import ErrorStatus

# TODO update the JSON string below
json = "{}"
# create an instance of ErrorStatus from a JSON string
error_status_instance = ErrorStatus.from_json(json)
# print the JSON string representation of the object
print(ErrorStatus.to_json())

# convert the object into a dict
error_status_dict = error_status_instance.to_dict()
# create an instance of ErrorStatus from a dict
error_status_from_dict = ErrorStatus.from_dict(error_status_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
