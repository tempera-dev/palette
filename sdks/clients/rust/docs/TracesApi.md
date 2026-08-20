# \TracesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**traces_period_get**](TracesApi.md#traces_period_get) | **GET** /v1/traces/{tenantId}/{traceId} |
[**traces_period_list**](TracesApi.md#traces_period_list) | **GET** /v1/traces/{tenantId} |



## traces_period_get

> models::TraceView traces_period_get(tenant_id, trace_id, unmask, reason, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**trace_id** | **String** | trace_id | [required] |
**unmask** | Option<**bool**> |  |  |
**reason** | Option<**String**> |  |  |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::TraceView**](TraceView.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## traces_period_list

> models::TraceListResponse traces_period_list(tenant_id, project_id, environment_id, trace_id, kind, status, started_after, started_before, model, release, min_cost_micros, max_cost_micros, min_latency_ms, max_latency_ms, page_size, page_token, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | Option<**String**> |  |  |
**environment_id** | Option<**String**> |  |  |
**trace_id** | Option<**String**> |  |  |
**kind** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**started_after** | Option<**String**> |  |  |
**started_before** | Option<**String**> |  |  |
**model** | Option<**String**> |  |  |
**release** | Option<**String**> |  |  |
**min_cost_micros** | Option<**i64**> |  |  |
**max_cost_micros** | Option<**i64**> |  |  |
**min_latency_ms** | Option<**i64**> |  |  |
**max_latency_ms** | Option<**i64**> |  |  |
**page_size** | Option<**i32**> |  |  |
**page_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::TraceListResponse**](TraceListResponse.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
