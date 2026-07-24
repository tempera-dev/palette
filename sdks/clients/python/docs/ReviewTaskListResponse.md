# ReviewTaskListResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**next_page_token** | **str** |  | [optional]
**tasks** | [**List[ReviewTask]**](ReviewTask.md) |  |

## Example

```python
from palette_client.models.review_task_list_response import ReviewTaskListResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ReviewTaskListResponse from a JSON string
review_task_list_response_instance = ReviewTaskListResponse.from_json(json)
# print the JSON string representation of the object
print(ReviewTaskListResponse.to_json())

# convert the object into a dict
review_task_list_response_dict = review_task_list_response_instance.to_dict()
# create an instance of ReviewTaskListResponse from a dict
review_task_list_response_from_dict = ReviewTaskListResponse.from_dict(review_task_list_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
