# \SearchApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_period_spans**](SearchApi.md#search_period_spans) | **GET** /v1/search/{tenantId}/spans |



## search_period_spans

> models::SearchSpanListResponse search_period_spans(tenant_id, q, project_id, environment_id, trace_id, span_id, kind, status, model, tool, page_size, page_token, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**q** | Option<**String**> |  |  |
**project_id** | Option<**String**> |  |  |
**environment_id** | Option<**String**> |  |  |
**trace_id** | Option<**String**> |  |  |
**span_id** | Option<**String**> |  |  |
**kind** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**model** | Option<**String**> |  |  |
**tool** | Option<**String**> |  |  |
**page_size** | Option<**i32**> |  |  |
**page_token** | Option<**String**> |  |  |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::SearchSpanListResponse**](SearchSpanListResponse.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
