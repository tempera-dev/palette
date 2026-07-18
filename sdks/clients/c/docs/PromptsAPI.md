# PromptsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PromptsAPI_promptsAddVersion**](PromptsAPI.md#PromptsAPI_promptsAddVersion) | **POST** /v1/prompts/{tenant_id}/{project_id}/{prompt_id}/versions |
[**PromptsAPI_promptsCreate**](PromptsAPI.md#PromptsAPI_promptsCreate) | **POST** /v1/prompts/{tenant_id}/{project_id} |
[**PromptsAPI_promptsDiffVersions**](PromptsAPI.md#PromptsAPI_promptsDiffVersions) | **GET** /v1/prompts/{tenant_id}/{project_id}/{prompt_id}/diff |
[**PromptsAPI_promptsGet**](PromptsAPI.md#PromptsAPI_promptsGet) | **GET** /v1/prompts/{tenant_id}/{project_id}/{prompt_id} |
[**PromptsAPI_promptsList**](PromptsAPI.md#PromptsAPI_promptsList) | **GET** /v1/prompts/{tenant_id}/{project_id} |
[**PromptsAPI_promptsListVersions**](PromptsAPI.md#PromptsAPI_promptsListVersions) | **GET** /v1/prompts/{tenant_id}/{project_id}/{prompt_id}/versions |


# **PromptsAPI_promptsAddVersion**
```c
prompt_version_t* PromptsAPI_promptsAddVersion(apiClient_t *apiClient, char *tenant_id, char *project_id, char *prompt_id, add_prompt_version_request_t *add_prompt_version_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**prompt_id** | **char \*** | prompt_id |
**add_prompt_version_request** | **[add_prompt_version_request_t](add_prompt_version_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[prompt_version_t](prompt_version.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **PromptsAPI_promptsCreate**
```c
created_prompt_t* PromptsAPI_promptsCreate(apiClient_t *apiClient, char *tenant_id, char *project_id, create_prompt_request_t *create_prompt_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**create_prompt_request** | **[create_prompt_request_t](create_prompt_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[created_prompt_t](created_prompt.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **PromptsAPI_promptsDiffVersions**
```c
prompt_version_diff_t* PromptsAPI_promptsDiffVersions(apiClient_t *apiClient, char *tenant_id, char *project_id, char *prompt_id, char *from, char *to, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**prompt_id** | **char \*** | prompt_id |
**from** | **char \*** |  |
**to** | **char \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[prompt_version_diff_t](prompt_version_diff.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **PromptsAPI_promptsGet**
```c
prompt_t* PromptsAPI_promptsGet(apiClient_t *apiClient, char *tenant_id, char *project_id, char *prompt_id, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**prompt_id** | **char \*** | prompt_id |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[prompt_t](prompt.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **PromptsAPI_promptsList**
```c
prompt_list_response_t* PromptsAPI_promptsList(apiClient_t *apiClient, char *tenant_id, char *project_id, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[prompt_list_response_t](prompt_list_response.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **PromptsAPI_promptsListVersions**
```c
prompt_version_list_response_t* PromptsAPI_promptsListVersions(apiClient_t *apiClient, char *tenant_id, char *project_id, char *prompt_id, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenant_id** | **char \*** | tenant_id |
**project_id** | **char \*** | project_id |
**prompt_id** | **char \*** | prompt_id |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[prompt_version_list_response_t](prompt_version_list_response.md) *


### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
