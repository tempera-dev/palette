# TemperaEvidenceReceipt


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | **bool** |  |
**declared_content_sha256** | **str** |  |
**external_id** | **str** |  |
**kind** | [**ExternalEvalEvidenceKind**](ExternalEvalEvidenceKind.md) |  |
**project_id** | **str** |  |
**public_key_sha256** | **str** |  |
**schema_version** | **str** |  |
**signature_sha256** | **str** |  |
**signed_payload_sha256** | **str** |  |
**source_schema_version** | **str** |  |
**stored_at** | **datetime** |  |
**summary** | [**TemperaEvidenceSummary**](TemperaEvidenceSummary.md) |  |
**tenant_id** | **str** |  |

## Example

```python
from palette_client.models.tempera_evidence_receipt import TemperaEvidenceReceipt

# TODO update the JSON string below
json = "{}"
# create an instance of TemperaEvidenceReceipt from a JSON string
tempera_evidence_receipt_instance = TemperaEvidenceReceipt.from_json(json)
# print the JSON string representation of the object
print(TemperaEvidenceReceipt.to_json())

# convert the object into a dict
tempera_evidence_receipt_dict = tempera_evidence_receipt_instance.to_dict()
# create an instance of TemperaEvidenceReceipt from a dict
tempera_evidence_receipt_from_dict = TemperaEvidenceReceipt.from_dict(tempera_evidence_receipt_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
