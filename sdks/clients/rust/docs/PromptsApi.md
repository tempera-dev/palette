# \PromptsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**prompts_period_add_version**](PromptsApi.md#prompts_period_add_version) | **POST** /v1/prompts/{tenant_id}/{project_id}/{prompt_id}/versions |
[**prompts_period_create**](PromptsApi.md#prompts_period_create) | **POST** /v1/prompts/{tenant_id}/{project_id} |
[**prompts_period_diff_versions**](PromptsApi.md#prompts_period_diff_versions) | **GET** /v1/prompts/{tenant_id}/{project_id}/{prompt_id}/diff |
[**prompts_period_get**](PromptsApi.md#prompts_period_get) | **GET** /v1/prompts/{tenant_id}/{project_id}/{prompt_id} |
[**prompts_period_list**](PromptsApi.md#prompts_period_list) | **GET** /v1/prompts/{tenant_id}/{project_id} |
[**prompts_period_list_versions**](PromptsApi.md#prompts_period_list_versions) | **GET** /v1/prompts/{tenant_id}/{project_id}/{prompt_id}/versions |



## prompts_period_add_version

> models::PromptVersion prompts_period_add_version(tenant_id, project_id, prompt_id, add_prompt_version_request, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**prompt_id** | **String** | prompt_id | [required] |
**add_prompt_version_request** | [**AddPromptVersionRequest**](AddPromptVersionRequest.md) |  | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::PromptVersion**](PromptVersion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prompts_period_create

> models::CreatedPrompt prompts_period_create(tenant_id, project_id, create_prompt_request, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**create_prompt_request** | [**CreatePromptRequest**](CreatePromptRequest.md) |  | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::CreatedPrompt**](CreatedPrompt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prompts_period_diff_versions

> models::PromptVersionDiff prompts_period_diff_versions(tenant_id, project_id, prompt_id, from, to, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**prompt_id** | **String** | prompt_id | [required] |
**from** | **String** |  | [required] |
**to** | **String** |  | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::PromptVersionDiff**](PromptVersionDiff.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prompts_period_get

> models::Prompt prompts_period_get(tenant_id, project_id, prompt_id, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**prompt_id** | **String** | prompt_id | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::Prompt**](Prompt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prompts_period_list

> models::PromptListResponse prompts_period_list(tenant_id, project_id, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::PromptListResponse**](PromptListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prompts_period_list_versions

> models::PromptVersionListResponse prompts_period_list_versions(tenant_id, project_id, prompt_id, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**prompt_id** | **String** | prompt_id | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::PromptVersionListResponse**](PromptVersionListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
