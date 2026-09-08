# CanonicalSpanOutputRef


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artifact_id** | **str** |  |
**mime_type** | **str** |  |
**redaction_class** | [**RedactionClass**](RedactionClass.md) |  |
**sha256** | **str** |  |
**size_bytes** | **int** |  |
**uri** | **str** |  |

## Example

```python
from palette_client.models.canonical_span_output_ref import CanonicalSpanOutputRef

# TODO update the JSON string below
json = "{}"
# create an instance of CanonicalSpanOutputRef from a JSON string
canonical_span_output_ref_instance = CanonicalSpanOutputRef.from_json(json)
# print the JSON string representation of the object
print(CanonicalSpanOutputRef.to_json())

# convert the object into a dict
canonical_span_output_ref_dict = canonical_span_output_ref_instance.to_dict()
# create an instance of CanonicalSpanOutputRef from a dict
canonical_span_output_ref_from_dict = CanonicalSpanOutputRef.from_dict(canonical_span_output_ref_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
