# IngestAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**IngestAPI_ingestDrainTraceIngested**](IngestAPI.md#IngestAPI_ingestDrainTraceIngested) | **POST** /v1/ingest/{tenantId}/{projectId}/trace-ingested/drain |
[**IngestAPI_ingestDrainTraceWrites**](IngestAPI.md#IngestAPI_ingestDrainTraceWrites) | **POST** /v1/ingest/{tenantId}/{projectId}/trace-writes/drain |
[**IngestAPI_ingestGetQueueStatus**](IngestAPI.md#IngestAPI_ingestGetQueueStatus) | **GET** /v1/ingest/{tenantId}/{projectId}/queue |
[**IngestAPI_ingestImportSource**](IngestAPI.md#IngestAPI_ingestImportSource) | **POST** /v1/import/{tenantId}/{projectId}/{environmentId} |
[**IngestAPI_ingestNative**](IngestAPI.md#IngestAPI_ingestNative) | **POST** /v1/traces/native |
[**IngestAPI_ingestOtlp**](IngestAPI.md#IngestAPI_ingestOtlp) | **POST** /v1/otlp/{tenantId}/{projectId}/{environmentId}/v1/traces |
[**IngestAPI_ingestOtlpJsonCollector**](IngestAPI.md#IngestAPI_ingestOtlpJsonCollector) | **POST** /v1/traces |
[**IngestAPI_ingestReconcileTrace**](IngestAPI.md#IngestAPI_ingestReconcileTrace) | **POST** /v1/ingest/{tenantId}/{projectId}/traces/{traceId}/reconcile |
[**IngestAPI_ingestReplayDeadLetter**](IngestAPI.md#IngestAPI_ingestReplayDeadLetter) | **POST** /v1/ingest/{tenantId}/{projectId}/dead-letters/{messageId}/replay |


# **IngestAPI_ingestDrainTraceIngested**
```c
trace_ingested_drain_report_t* IngestAPI_ingestDrainTraceIngested(apiClient_t *apiClient, char *tenantId, char *projectId, int *limit, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**limit** | **int \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[trace_ingested_drain_report_t](trace_ingested_drain_report.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **IngestAPI_ingestDrainTraceWrites**
```c
trace_write_drain_report_t* IngestAPI_ingestDrainTraceWrites(apiClient_t *apiClient, char *tenantId, char *projectId, int *limit, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**limit** | **int \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[trace_write_drain_report_t](trace_write_drain_report.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **IngestAPI_ingestGetQueueStatus**
```c
ingest_queue_status_t* IngestAPI_ingestGetQueueStatus(apiClient_t *apiClient, char *tenantId, char *projectId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[ingest_queue_status_t](ingest_queue_status.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **IngestAPI_ingestImportSource**
```c
ingest_outcome_t* IngestAPI_ingestImportSource(apiClient_t *apiClient, char *tenantId, char *projectId, char *environmentId, import_source_http_request_t *import_source_http_request, char *durability, char *authorization, char *x_palette_api_key);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**environmentId** | **char \*** | environment_id |
**import_source_http_request** | **[import_source_http_request_t](import_source_http_request.md) \*** |  |
**durability** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]

### Return type

[ingest_outcome_t](ingest_outcome.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **IngestAPI_ingestNative**
```c
ingest_outcome_t* IngestAPI_ingestNative(apiClient_t *apiClient, native_ingest_request_t *native_ingest_request, char *durability, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**native_ingest_request** | **[native_ingest_request_t](native_ingest_request.md) \*** |  |
**durability** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[ingest_outcome_t](ingest_outcome.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **IngestAPI_ingestOtlp**
```c
otlp_ingest_outcome_t* IngestAPI_ingestOtlp(apiClient_t *apiClient, char *tenantId, char *projectId, char *environmentId, char *durability, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**environmentId** | **char \*** | environment_id |
**durability** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[otlp_ingest_outcome_t](otlp_ingest_outcome.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **IngestAPI_ingestOtlpJsonCollector**
```c
otlp_ingest_outcome_t* IngestAPI_ingestOtlpJsonCollector(apiClient_t *apiClient, char *durability, char *authorization, char *x_palette_api_key, char *x_palette_tenant_id, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**durability** | **char \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_tenant_id** | **char \*** | Tenant scope override for collector-style OTLP JSON | [optional]
**x_palette_project_id** | **char \*** | Project scope override for collector-style OTLP JSON | [optional]
**x_palette_environment_id** | **char \*** | Environment scope override for collector-style OTLP JSON | [optional]

### Return type

[otlp_ingest_outcome_t](otlp_ingest_outcome.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **IngestAPI_ingestReconcileTrace**
```c
trace_ingested_reconcile_report_t* IngestAPI_ingestReconcileTrace(apiClient_t *apiClient, char *tenantId, char *projectId, char *traceId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**traceId** | **char \*** | trace_id |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[trace_ingested_reconcile_report_t](trace_ingested_reconcile_report.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **IngestAPI_ingestReplayDeadLetter**
```c
dead_letter_replay_report_t* IngestAPI_ingestReplayDeadLetter(apiClient_t *apiClient, char *tenantId, char *projectId, char *messageId, int *resetAttempts, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**messageId** | **char \*** | message_id |
**resetAttempts** | **int \*** |  | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[dead_letter_replay_report_t](dead_letter_replay_report.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
