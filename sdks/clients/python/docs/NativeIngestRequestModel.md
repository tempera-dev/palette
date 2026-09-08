# NativeIngestRequestModel


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **str** |  |
**provider** | **str** |  |

## Example

```python
from palette_client.models.native_ingest_request_model import NativeIngestRequestModel

# TODO update the JSON string below
json = "{}"
# create an instance of NativeIngestRequestModel from a JSON string
native_ingest_request_model_instance = NativeIngestRequestModel.from_json(json)
# print the JSON string representation of the object
print(NativeIngestRequestModel.to_json())

# convert the object into a dict
native_ingest_request_model_dict = native_ingest_request_model_instance.to_dict()
# create an instance of NativeIngestRequestModel from a dict
native_ingest_request_model_from_dict = NativeIngestRequestModel.from_dict(native_ingest_request_model_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
