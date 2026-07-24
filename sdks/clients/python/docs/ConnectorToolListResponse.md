# ConnectorToolListResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**next_page_token** | **str** |  | [optional]
**tools** | [**List[ConnectorTool]**](ConnectorTool.md) |  |

## Example

```python
from palette_client.models.connector_tool_list_response import ConnectorToolListResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ConnectorToolListResponse from a JSON string
connector_tool_list_response_instance = ConnectorToolListResponse.from_json(json)
# print the JSON string representation of the object
print(ConnectorToolListResponse.to_json())

# convert the object into a dict
connector_tool_list_response_dict = connector_tool_list_response_instance.to_dict()
# create an instance of ConnectorToolListResponse from a dict
connector_tool_list_response_from_dict = ConnectorToolListResponse.from_dict(connector_tool_list_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
