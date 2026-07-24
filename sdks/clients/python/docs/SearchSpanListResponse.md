# SearchSpanListResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hits** | [**List[SearchHit]**](SearchHit.md) |  |
**next_page_token** | **str** |  | [optional]

## Example

```python
from palette_client.models.search_span_list_response import SearchSpanListResponse

# TODO update the JSON string below
json = "{}"
# create an instance of SearchSpanListResponse from a JSON string
search_span_list_response_instance = SearchSpanListResponse.from_json(json)
# print the JSON string representation of the object
print(SearchSpanListResponse.to_json())

# convert the object into a dict
search_span_list_response_dict = search_span_list_response_instance.to_dict()
# create an instance of SearchSpanListResponse from a dict
search_span_list_response_from_dict = SearchSpanListResponse.from_dict(search_span_list_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
