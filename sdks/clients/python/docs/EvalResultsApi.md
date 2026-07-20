# palette_client.EvalResultsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**eval_results_get_tempera_evidence**](EvalResultsApi.md#eval_results_get_tempera_evidence) | **GET** /v1/eval-results/{tenant_id}/{project_id}/tempera/{kind}/{external_id} |
[**eval_results_import_tempera_bundle**](EvalResultsApi.md#eval_results_import_tempera_bundle) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/bundles |
[**eval_results_record_tempera_decision**](EvalResultsApi.md#eval_results_record_tempera_decision) | **POST** /v1/eval-results/{tenant_id}/{project_id}/tempera/decisions |


# **eval_results_get_tempera_evidence**
> TemperaEvidenceReceipt eval_results_get_tempera_evidence(tenant_id, project_id, kind, external_id, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.tempera_evidence_receipt import TemperaEvidenceReceipt
from palette_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = palette_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with palette_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = palette_client.EvalResultsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    kind = 'kind_example' # str | result_bundle or ab_decision
    external_id = 'external_id_example' # str | Bundle or experiment id
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.eval_results_get_tempera_evidence(tenant_id, project_id, kind, external_id, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of EvalResultsApi->eval_results_get_tempera_evidence:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling EvalResultsApi->eval_results_get_tempera_evidence: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **kind** | **str**| result_bundle or ab_decision |
 **external_id** | **str**| Bundle or experiment id |
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Read a scoped external evidence receipt |  -  |
**400** | Invalid evidence kind or identifier |  -  |
**401** | Missing or invalid credentials |  -  |
**403** | Credentials lack the required scope |  -  |
**404** | Evidence not found in this tenant/project |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **eval_results_import_tempera_bundle**
> TemperaEvidenceReceipt eval_results_import_tempera_bundle(tenant_id, project_id, import_tempera_evidence_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.import_tempera_evidence_request import ImportTemperaEvidenceRequest
from palette_client.models.tempera_evidence_receipt import TemperaEvidenceReceipt
from palette_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = palette_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with palette_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = palette_client.EvalResultsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    import_tempera_evidence_request = palette_client.ImportTemperaEvidenceRequest() # ImportTemperaEvidenceRequest |
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.eval_results_import_tempera_bundle(tenant_id, project_id, import_tempera_evidence_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of EvalResultsApi->eval_results_import_tempera_bundle:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling EvalResultsApi->eval_results_import_tempera_bundle: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **import_tempera_evidence_request** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md)|  |
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Idempotently store a verified official Tempera result bundle |  -  |
**400** | Malformed, non-canonical, unsafe, or signature-invalid evidence |  -  |
**401** | Missing or invalid credentials |  -  |
**403** | Credentials lack the required scope or the evidence key is not trusted |  -  |
**409** | The external id already binds different content |  -  |
**413** | Evidence exceeds the request limit |  -  |
**422** | Request body does not match the schema |  -  |
**503** | No Tempera evaluation release-key trust anchor is configured |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **eval_results_record_tempera_decision**
> TemperaEvidenceReceipt eval_results_record_tempera_decision(tenant_id, project_id, import_tempera_evidence_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.import_tempera_evidence_request import ImportTemperaEvidenceRequest
from palette_client.models.tempera_evidence_receipt import TemperaEvidenceReceipt
from palette_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = palette_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with palette_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = palette_client.EvalResultsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    import_tempera_evidence_request = palette_client.ImportTemperaEvidenceRequest() # ImportTemperaEvidenceRequest |
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.eval_results_record_tempera_decision(tenant_id, project_id, import_tempera_evidence_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of EvalResultsApi->eval_results_record_tempera_decision:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling EvalResultsApi->eval_results_record_tempera_decision: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **import_tempera_evidence_request** | [**ImportTemperaEvidenceRequest**](ImportTemperaEvidenceRequest.md)|  |
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**TemperaEvidenceReceipt**](TemperaEvidenceReceipt.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Idempotently store a verified preregistered Tempera A/B decision |  -  |
**400** | Malformed, non-canonical, unsafe, or signature-invalid evidence |  -  |
**401** | Missing or invalid credentials |  -  |
**403** | Credentials lack the required scope or the evidence key is not trusted |  -  |
**409** | The external id already binds different content |  -  |
**413** | Evidence exceeds the request limit |  -  |
**422** | Request body does not match the schema |  -  |
**503** | No Tempera evaluation release-key trust anchor is configured |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
