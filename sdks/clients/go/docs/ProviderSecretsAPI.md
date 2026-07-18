# \ProviderSecretsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ProviderSecretsCreate**](ProviderSecretsAPI.md#ProviderSecretsCreate) | **Post** /v1/provider-secrets/{tenant_id}/{project_id} |
[**ProviderSecretsList**](ProviderSecretsAPI.md#ProviderSecretsList) | **Get** /v1/provider-secrets/{tenant_id}/{project_id} |
[**ProviderSecretsRevoke**](ProviderSecretsAPI.md#ProviderSecretsRevoke) | **Post** /v1/provider-secrets/{tenant_id}/{project_id}/{provider_secret_id}/revoke |



## ProviderSecretsCreate

> ProviderSecretMetadata ProviderSecretsCreate(ctx, tenantId, projectId).CreateProviderSecretHttpRequest(createProviderSecretHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/paletteclient"
)

func main() {
	tenantId := "tenantId_example" // string | tenant_id
	projectId := "projectId_example" // string | project_id
	createProviderSecretHttpRequest := *openapiclient.NewCreateProviderSecretHttpRequest("DisplayName_example", "Provider_example", "SecretValue_example") // CreateProviderSecretHttpRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ProviderSecretsAPI.ProviderSecretsCreate(context.Background(), tenantId, projectId).CreateProviderSecretHttpRequest(createProviderSecretHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ProviderSecretsAPI.ProviderSecretsCreate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ProviderSecretsCreate`: ProviderSecretMetadata
	fmt.Fprintf(os.Stdout, "Response from `ProviderSecretsAPI.ProviderSecretsCreate`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |

### Other Parameters

Other parameters are passed through a pointer to a apiProviderSecretsCreateRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **createProviderSecretHttpRequest** | [**CreateProviderSecretHttpRequest**](CreateProviderSecretHttpRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**ProviderSecretMetadata**](ProviderSecretMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ProviderSecretsList

> []ProviderSecretMetadata ProviderSecretsList(ctx, tenantId, projectId).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/paletteclient"
)

func main() {
	tenantId := "tenantId_example" // string | tenant_id
	projectId := "projectId_example" // string | project_id
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ProviderSecretsAPI.ProviderSecretsList(context.Background(), tenantId, projectId).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ProviderSecretsAPI.ProviderSecretsList``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ProviderSecretsList`: []ProviderSecretMetadata
	fmt.Fprintf(os.Stdout, "Response from `ProviderSecretsAPI.ProviderSecretsList`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |

### Other Parameters

Other parameters are passed through a pointer to a apiProviderSecretsListRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**[]ProviderSecretMetadata**](ProviderSecretMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ProviderSecretsRevoke

> RevokedProviderSecret ProviderSecretsRevoke(ctx, tenantId, projectId, providerSecretId).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/paletteclient"
)

func main() {
	tenantId := "tenantId_example" // string | tenant_id
	projectId := "projectId_example" // string | project_id
	providerSecretId := "providerSecretId_example" // string | provider_secret_id
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ProviderSecretsAPI.ProviderSecretsRevoke(context.Background(), tenantId, projectId, providerSecretId).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ProviderSecretsAPI.ProviderSecretsRevoke``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ProviderSecretsRevoke`: RevokedProviderSecret
	fmt.Fprintf(os.Stdout, "Response from `ProviderSecretsAPI.ProviderSecretsRevoke`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |
**providerSecretId** | **string** | provider_secret_id |

### Other Parameters

Other parameters are passed through a pointer to a apiProviderSecretsRevokeRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**RevokedProviderSecret**](RevokedProviderSecret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
