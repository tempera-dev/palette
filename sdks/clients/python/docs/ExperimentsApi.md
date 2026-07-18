# palette_client.ExperimentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**experiments_run_deterministic**](ExperimentsApi.md#experiments_run_deterministic) | **POST** /v1/experiments/{tenant_id}/{project_id}/{dataset_id}/versions/{version_id}/deterministic |
[**experiments_run_judge**](ExperimentsApi.md#experiments_run_judge) | **POST** /v1/experiments/{tenant_id}/{project_id}/{dataset_id}/versions/{version_id}/judge |


# **experiments_run_deterministic**
> ExperimentRunReport experiments_run_deterministic(tenant_id, project_id, dataset_id, version_id, run_experiment_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.experiment_run_report import ExperimentRunReport
from palette_client.models.run_experiment_request import RunExperimentRequest
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
    api_instance = palette_client.ExperimentsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    dataset_id = 'dataset_id_example' # str | dataset_id
    version_id = 'version_id_example' # str | version_id
    run_experiment_request = palette_client.RunExperimentRequest() # RunExperimentRequest |
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.experiments_run_deterministic(tenant_id, project_id, dataset_id, version_id, run_experiment_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of ExperimentsApi->experiments_run_deterministic:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExperimentsApi->experiments_run_deterministic: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **dataset_id** | **str**| dataset_id |
 **version_id** | **str**| version_id |
 **run_experiment_request** | [**RunExperimentRequest**](RunExperimentRequest.md)|  |
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**ExperimentRunReport**](ExperimentRunReport.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Run a deterministic experiment |  -  |
**400** | Invalid request, scope, or filter |  -  |
**401** | Missing or invalid credentials |  -  |
**403** | Credentials lack the required scope |  -  |
**404** | Resource not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **experiments_run_judge**
> ExperimentRunReport experiments_run_judge(tenant_id, project_id, dataset_id, version_id, run_judge_experiment_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.experiment_run_report import ExperimentRunReport
from palette_client.models.run_judge_experiment_request import RunJudgeExperimentRequest
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
    api_instance = palette_client.ExperimentsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    dataset_id = 'dataset_id_example' # str | dataset_id
    version_id = 'version_id_example' # str | version_id
    run_judge_experiment_request = palette_client.RunJudgeExperimentRequest() # RunJudgeExperimentRequest |
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.experiments_run_judge(tenant_id, project_id, dataset_id, version_id, run_judge_experiment_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of ExperimentsApi->experiments_run_judge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ExperimentsApi->experiments_run_judge: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **dataset_id** | **str**| dataset_id |
 **version_id** | **str**| version_id |
 **run_judge_experiment_request** | [**RunJudgeExperimentRequest**](RunJudgeExperimentRequest.md)|  |
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**ExperimentRunReport**](ExperimentRunReport.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Run a judge experiment |  -  |
**400** | Invalid request, scope, or filter |  -  |
**401** | Missing or invalid credentials |  -  |
**403** | Credentials lack the required scope |  -  |
**404** | Resource not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
