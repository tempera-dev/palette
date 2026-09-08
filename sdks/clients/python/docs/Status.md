# Status

The google.rpc.Status error envelope required of every Tempera API (AIP-193).

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | [**StatusError**](StatusError.md) |  |

## Example

```python
from palette_client.models.status import Status

# TODO update the JSON string below
json = "{}"
# create an instance of Status from a JSON string
status_instance = Status.from_json(json)
# print the JSON string representation of the object
print(Status.to_json())

# convert the object into a dict
status_dict = status_instance.to_dict()
# create an instance of Status from a dict
status_from_dict = Status.from_dict(status_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
