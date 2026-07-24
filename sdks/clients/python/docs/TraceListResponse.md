# TraceListResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**next_page_token** | **str** |  | [optional]
**runs** | [**List[RunSummary]**](RunSummary.md) |  |

## Example

```python
from palette_client.models.trace_list_response import TraceListResponse

# TODO update the JSON string below
json = "{}"
# create an instance of TraceListResponse from a JSON string
trace_list_response_instance = TraceListResponse.from_json(json)
# print the JSON string representation of the object
print(TraceListResponse.to_json())

# convert the object into a dict
trace_list_response_dict = trace_list_response_instance.to_dict()
# create an instance of TraceListResponse from a dict
trace_list_response_from_dict = TraceListResponse.from_dict(trace_list_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
