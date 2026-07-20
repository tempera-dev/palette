# TemperaEvidenceSummary


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**experiment_id** | **str** |  | [optional]
**run_id** | **str** |  | [optional]
**split** | **str** |  | [optional]
**suite_id** | **str** |  | [optional]
**suite_version** | **str** |  | [optional]
**verdict** | **str** |  | [optional]

## Example

```python
from palette_client.models.tempera_evidence_summary import TemperaEvidenceSummary

# TODO update the JSON string below
json = "{}"
# create an instance of TemperaEvidenceSummary from a JSON string
tempera_evidence_summary_instance = TemperaEvidenceSummary.from_json(json)
# print the JSON string representation of the object
print(TemperaEvidenceSummary.to_json())

# convert the object into a dict
tempera_evidence_summary_dict = tempera_evidence_summary_instance.to_dict()
# create an instance of TemperaEvidenceSummary from a dict
tempera_evidence_summary_from_dict = TemperaEvidenceSummary.from_dict(tempera_evidence_summary_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
