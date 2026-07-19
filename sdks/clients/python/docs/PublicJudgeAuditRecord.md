# PublicJudgeAuditRecord

Client-facing judge ledger row. The backing `provider`, the `provider_secret_id`, and our raw `provider_cost` are INTERNAL (staff-only) and must never reach a customer — exposing `provider_cost` alongside `charged_cost` would also leak our margin (billing-credits-contract §11). Only customer-facing fields appear here, including `charged_cost` (the amount the customer actually pays).

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cached** | **bool** |  |
**charged_cost** | [**Money**](Money.md) |  |
**created_at** | **datetime** |  |
**evaluator_id** | **str** |  |
**judge_call_id** | **str** |  |
**model** | **str** |  |
**project_id** | **str** |  |
**request_hash** | **str** |  |
**response_hash** | **str** |  |
**score** | **float** |  |
**tenant_id** | **str** |  |

## Example

```python
from palette_client.models.public_judge_audit_record import PublicJudgeAuditRecord

# TODO update the JSON string below
json = "{}"
# create an instance of PublicJudgeAuditRecord from a JSON string
public_judge_audit_record_instance = PublicJudgeAuditRecord.from_json(json)
# print the JSON string representation of the object
print(PublicJudgeAuditRecord.to_json())

# convert the object into a dict
public_judge_audit_record_dict = public_judge_audit_record_instance.to_dict()
# create an instance of PublicJudgeAuditRecord from a dict
public_judge_audit_record_from_dict = PublicJudgeAuditRecord.from_dict(public_judge_audit_record_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
