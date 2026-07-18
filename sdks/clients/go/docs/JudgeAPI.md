# \JudgeAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**JudgeEvaluate**](JudgeAPI.md#JudgeEvaluate) | **Post** /v1/judge/{tenant_id}/{project_id}/evaluate |
[**JudgeListLedger**](JudgeAPI.md#JudgeListLedger) | **Get** /v1/judge/{tenant_id}/{project_id}/ledger |



## JudgeEvaluate

> JudgeBrokerOutcome JudgeEvaluate(ctx, tenantId, projectId).RunJudgeEvalHttpRequest(runJudgeEvalHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	runJudgeEvalHttpRequest := *openapiclient.NewRunJudgeEvalHttpRequest(*openapiclient.NewEvaluationCase(interface{}(123), interface{}(123)), *openapiclient.NewEvaluatorSpec("Id_example", openapiclient.EvaluatorKind{EvaluatorKindOneOf: openapiclient.NewEvaluatorKindOneOf("Type_example")}, openapiclient.EvaluatorLane("deterministic_wasi")), "ProviderSecretId_example") // RunJudgeEvalHttpRequest |
	authorization := "authorization_example" // string | Bearer API token for strict auth (optional)
	xPaletteApiKey := "xPaletteApiKey_example" // string | API key alternative for strict auth (optional)
	xPaletteProjectId := "xPaletteProjectId_example" // string | Strict-auth project scope (optional)
	xPaletteEnvironmentId := "xPaletteEnvironmentId_example" // string | Strict-auth environment scope (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.JudgeAPI.JudgeEvaluate(context.Background(), tenantId, projectId).RunJudgeEvalHttpRequest(runJudgeEvalHttpRequest).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JudgeAPI.JudgeEvaluate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `JudgeEvaluate`: JudgeBrokerOutcome
	fmt.Fprintf(os.Stdout, "Response from `JudgeAPI.JudgeEvaluate`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |

### Other Parameters

Other parameters are passed through a pointer to a apiJudgeEvaluateRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **runJudgeEvalHttpRequest** | [**RunJudgeEvalHttpRequest**](RunJudgeEvalHttpRequest.md) |  |
 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**JudgeBrokerOutcome**](JudgeBrokerOutcome.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## JudgeListLedger

> []JudgeAuditRecord JudgeListLedger(ctx, tenantId, projectId).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()



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
	resp, r, err := apiClient.JudgeAPI.JudgeListLedger(context.Background(), tenantId, projectId).Authorization(authorization).XPaletteApiKey(xPaletteApiKey).XPaletteProjectId(xPaletteProjectId).XPaletteEnvironmentId(xPaletteEnvironmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JudgeAPI.JudgeListLedger``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `JudgeListLedger`: []JudgeAuditRecord
	fmt.Fprintf(os.Stdout, "Response from `JudgeAPI.JudgeListLedger`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | tenant_id |
**projectId** | **string** | project_id |

### Other Parameters

Other parameters are passed through a pointer to a apiJudgeListLedgerRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **authorization** | **string** | Bearer API token for strict auth |
 **xPaletteApiKey** | **string** | API key alternative for strict auth |
 **xPaletteProjectId** | **string** | Strict-auth project scope |
 **xPaletteEnvironmentId** | **string** | Strict-auth environment scope |

### Return type

[**[]JudgeAuditRecord**](JudgeAuditRecord.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
