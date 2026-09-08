# NativeIngestRequestTokens


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cache_read** | **int** |  |
**input** | **int** |  |
**output** | **int** |  |
**reasoning** | **int** |  |

## Example

```python
from palette_client.models.native_ingest_request_tokens import NativeIngestRequestTokens

# TODO update the JSON string below
json = "{}"
# create an instance of NativeIngestRequestTokens from a JSON string
native_ingest_request_tokens_instance = NativeIngestRequestTokens.from_json(json)
# print the JSON string representation of the object
print(NativeIngestRequestTokens.to_json())

# convert the object into a dict
native_ingest_request_tokens_dict = native_ingest_request_tokens_instance.to_dict()
# create an instance of NativeIngestRequestTokens from a dict
native_ingest_request_tokens_from_dict = NativeIngestRequestTokens.from_dict(native_ingest_request_tokens_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
