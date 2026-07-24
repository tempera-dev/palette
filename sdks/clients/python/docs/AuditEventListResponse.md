# AuditEventListResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**events** | [**List[AuditEvent]**](AuditEvent.md) |  |
**next_page_token** | **str** |  | [optional]

## Example

```python
from palette_client.models.audit_event_list_response import AuditEventListResponse

# TODO update the JSON string below
json = "{}"
# create an instance of AuditEventListResponse from a JSON string
audit_event_list_response_instance = AuditEventListResponse.from_json(json)
# print the JSON string representation of the object
print(AuditEventListResponse.to_json())

# convert the object into a dict
audit_event_list_response_dict = audit_event_list_response_instance.to_dict()
# create an instance of AuditEventListResponse from a dict
audit_event_list_response_from_dict = AuditEventListResponse.from_dict(audit_event_list_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
