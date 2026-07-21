# \EvalResultsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**EvalResultsGetTemperaEvidence**](EvalResultsAPI.md#EvalResultsGetTemperaEvidence) | **Get** /v1/eval-results/{tenant_id}/{project_id}/tempera/{kind}/{external_id} |
[**EvalResultsImportTemperaBundle**](EvalResultsAPI.md#EvalResultsImportTemperaBundle) | **Post** /v1/eval-results/{tenant_id}/{project_id}/tempera/bundles |
[**EvalResultsRecordTemperaDecision**](EvalResultsAPI.md#EvalResultsRecordTemperaDecision) | **Post** /v1/eval-results/{tenant_id}/{project_id}/tempera/decisions |



## EvalResultsGetTemperaEvidence

> TemperaEvidenceReceipt EvalResultsGetTemperaEvidence(ctx, tenantId, projectId, kind, externalId).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	kind := "kind_example" // string | result_bundle or ab_decision
	externalId := "externalId_example" // string | Bundle or experiment id
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.EvalResultsAPI.EvalResultsGetTemperaEvidence(context.Background(), tenantId, projectId, kind, externalId).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EvalResultsAPI.EvalResultsGetTemperaEvidence``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `EvalResultsGetTemperaEvidence`: TemperaEvidenceReceipt
	fmt.Fprintf(os.Stdout, "Response from `EvalResultsAPI.EvalResultsGetTemperaEvidence`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |
**kind** | **string** | result_bundle or ab_decision |
**externalId** | **string** | Bundle or experiment id |

### Other Parameters

Other parameters are passed through a pointer to a apiEvalResultsGetTemperaEvidenceRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------




 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## EvalResultsImportTemperaBundle

> TemperaEvidenceReceipt EvalResultsImportTemperaBundle(ctx, tenantId, projectId).ImportTemperaEvidenceRequest(importTemperaEvidenceRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	importTemperaEvidenceRequest := *openapiclient.NewImportTemperaEvidenceRequest("CanonicalJson_example", "PublicKeyPem_example", "SignatureBase64_example") // ImportTemperaEvidenceRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.EvalResultsAPI.EvalResultsImportTemperaBundle(context.Background(), tenantId, projectId).ImportTemperaEvidenceRequest(importTemperaEvidenceRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EvalResultsAPI.EvalResultsImportTemperaBundle``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `EvalResultsImportTemperaBundle`: TemperaEvidenceReceipt
	fmt.Fprintf(os.Stdout, "Response from `EvalResultsAPI.EvalResultsImportTemperaBundle`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |

### Other Parameters

Other parameters are passed through a pointer to a apiEvalResultsImportTemperaBundleRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **importTemperaEvidenceRequest** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## EvalResultsRecordTemperaDecision

> TemperaEvidenceReceipt EvalResultsRecordTemperaDecision(ctx, tenantId, projectId).ImportTemperaEvidenceRequest(importTemperaEvidenceRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	importTemperaEvidenceRequest := *openapiclient.NewImportTemperaEvidenceRequest("CanonicalJson_example", "PublicKeyPem_example", "SignatureBase64_example") // ImportTemperaEvidenceRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.EvalResultsAPI.EvalResultsRecordTemperaDecision(context.Background(), tenantId, projectId).ImportTemperaEvidenceRequest(importTemperaEvidenceRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EvalResultsAPI.EvalResultsRecordTemperaDecision``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `EvalResultsRecordTemperaDecision`: TemperaEvidenceReceipt
	fmt.Fprintf(os.Stdout, "Response from `EvalResultsAPI.EvalResultsRecordTemperaDecision`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |

### Other Parameters

Other parameters are passed through a pointer to a apiEvalResultsRecordTemperaDecisionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **importTemperaEvidenceRequest** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
