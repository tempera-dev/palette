# \GatesAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GatesCreate**](GatesAPI.md#GatesCreate) | **Post** /v1/gates/{tenant_id}/{project_id} |
[**GatesRun**](GatesAPI.md#GatesRun) | **Post** /v1/gates/{tenant_id}/{project_id}/{gate_id}/run |



## GatesCreate

> GateDefinition GatesCreate(ctx, tenantId, projectId).CreateGateRequest(createGateRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	createGateRequest := *openapiclient.NewCreateGateRequest("GateId_example", "Name_example") // CreateGateRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GatesAPI.GatesCreate(context.Background(), tenantId, projectId).CreateGateRequest(createGateRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GatesAPI.GatesCreate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GatesCreate`: GateDefinition
	fmt.Fprintf(os.Stdout, "Response from `GatesAPI.GatesCreate`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |

### Other Parameters

Other parameters are passed through a pointer to a apiGatesCreateRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **createGateRequest** | [**CreateGateRequest**](CreateGateRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**GateDefinition**](GateDefinition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GatesRun

> GateRunReport GatesRun(ctx, tenantId, projectId, gateId).RunGateRequest(runGateRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	gateId := "gateId_example" // string | gate_id
	runGateRequest := *openapiclient.NewRunGateRequest() // RunGateRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GatesAPI.GatesRun(context.Background(), tenantId, projectId, gateId).RunGateRequest(runGateRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GatesAPI.GatesRun``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GatesRun`: GateRunReport
	fmt.Fprintf(os.Stdout, "Response from `GatesAPI.GatesRun`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |
**gateId** | **string** | gate_id |

### Other Parameters

Other parameters are passed through a pointer to a apiGatesRunRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **runGateRequest** | [**RunGateRequest**](RunGateRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**GateRunReport**](GateRunReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
