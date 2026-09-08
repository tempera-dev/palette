# NativeIngestRequestCost


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount_micros** | **int** |  |
**currency** | [**Currency**](Currency.md) |  |

## Example

```python
from palette_client.models.native_ingest_request_cost import NativeIngestRequestCost

# TODO update the JSON string below
json = "{}"
# create an instance of NativeIngestRequestCost from a JSON string
native_ingest_request_cost_instance = NativeIngestRequestCost.from_json(json)
# print the JSON string representation of the object
print(NativeIngestRequestCost.to_json())

# convert the object into a dict
native_ingest_request_cost_dict = native_ingest_request_cost_instance.to_dict()
# create an instance of NativeIngestRequestCost from a dict
native_ingest_request_cost_from_dict = NativeIngestRequestCost.from_dict(native_ingest_request_cost_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
