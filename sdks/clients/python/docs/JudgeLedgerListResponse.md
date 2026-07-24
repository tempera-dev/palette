# JudgeLedgerListResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**next_page_token** | **str** |  | [optional]
**records** | [**List[PublicJudgeAuditRecord]**](PublicJudgeAuditRecord.md) |  |

## Example

```python
from palette_client.models.judge_ledger_list_response import JudgeLedgerListResponse

# TODO update the JSON string below
json = "{}"
# create an instance of JudgeLedgerListResponse from a JSON string
judge_ledger_list_response_instance = JudgeLedgerListResponse.from_json(json)
# print the JSON string representation of the object
print(JudgeLedgerListResponse.to_json())

# convert the object into a dict
judge_ledger_list_response_dict = judge_ledger_list_response_instance.to_dict()
# create an instance of JudgeLedgerListResponse from a dict
judge_ledger_list_response_from_dict = JudgeLedgerListResponse.from_dict(judge_ledger_list_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
