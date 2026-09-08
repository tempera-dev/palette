# native_ingest_request_t

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attributes** | **list_t*** |  |
**auth_context** | [**auth_context_t**](auth_context.md) \* |  | [optional]
**cost** | [**native_ingest_request_cost_t**](native_ingest_request_cost.md) \* |  | [optional]
**end_time** | **char \*** |  | [optional]
**idempotency_key** | **char \*** |  | [optional]
**input** | **any_type_t \*** |  | [optional]
**kind** | **char \*** | Canonical agent span kind such as agent.run or llm.call |
**model** | [**native_ingest_request_model_t**](native_ingest_request_model.md) \* |  | [optional]
**name** | **char \*** |  |
**output** | **any_type_t \*** |  | [optional]
**parent_span_id** | [**native_ingest_request_parent_span_id_t**](native_ingest_request_parent_span_id.md) \* |  | [optional]
**redaction_class** | **redaction_class_t \*** |  |
**scope** | [**tenant_scope_t**](tenant_scope.md) \* |  |
**seq** | **long** |  |
**span_id** | **char \*** |  |
**start_time** | **char \*** |  | [optional]
**status** | **span_status_t \*** |  |
**tokens** | [**native_ingest_request_tokens_t**](native_ingest_request_tokens.md) \* |  | [optional]
**trace_id** | **char \*** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
