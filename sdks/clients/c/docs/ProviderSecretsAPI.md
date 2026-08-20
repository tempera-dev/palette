# ProviderSecretsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ProviderSecretsAPI_providerSecretsCreate**](ProviderSecretsAPI.md#ProviderSecretsAPI_providerSecretsCreate) | **POST** /v1/provider-secrets/{tenantId}/{projectId} |
[**ProviderSecretsAPI_providerSecretsList**](ProviderSecretsAPI.md#ProviderSecretsAPI_providerSecretsList) | **GET** /v1/provider-secrets/{tenantId}/{projectId} |
[**ProviderSecretsAPI_providerSecretsRevoke**](ProviderSecretsAPI.md#ProviderSecretsAPI_providerSecretsRevoke) | **POST** /v1/provider-secrets/{tenantId}/{projectId}/{providerSecretId}/revoke |


# **ProviderSecretsAPI_providerSecretsCreate**
```c
provider_secret_metadata_t* ProviderSecretsAPI_providerSecretsCreate(apiClient_t *apiClient, char *tenantId, char *projectId, create_provider_secret_http_request_t *create_provider_secret_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**create_provider_secret_http_request** | **[create_provider_secret_http_request_t](create_provider_secret_http_request.md) \*** |  |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[provider_secret_metadata_t](provider_secret_metadata.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ProviderSecretsAPI_providerSecretsList**
```c
provider_secret_list_response_t* ProviderSecretsAPI_providerSecretsList(apiClient_t *apiClient, char *tenantId, char *projectId, int *pageSize, char *pageToken, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**pageSize** | **int \*** | Maximum number of resources to return. Zero selects the server default; values above the service maximum are coerced to that maximum. | [optional]
**pageToken** | **char \*** | Opaque continuation token returned by the preceding list request. | [optional]
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[provider_secret_list_response_t](provider_secret_list_response.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ProviderSecretsAPI_providerSecretsRevoke**
```c
revoked_provider_secret_t* ProviderSecretsAPI_providerSecretsRevoke(apiClient_t *apiClient, char *tenantId, char *projectId, char *providerSecretId, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);
```

### Parameters
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**apiClient** | **apiClient_t \*** | context containing the client configuration |
**tenantId** | **char \*** | tenant_id |
**projectId** | **char \*** | project_id |
**providerSecretId** | **char \*** | provider_secret_id |
**authorization** | **char \*** | Bearer API token for strict auth | [optional]
**x_palette_api_key** | **char \*** | API key alternative for strict auth | [optional]
**x_palette_project_id** | **char \*** | Strict-auth project scope | [optional]
**x_palette_environment_id** | **char \*** | Strict-auth environment scope | [optional]

### Return type

[revoked_provider_secret_t](revoked_provider_secret.md) *


### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
