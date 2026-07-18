# palette_client.SpansApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**spans_get**](SpansApi.md#spans_get) | **GET** /v1/spans/{tenant_id}/{trace_id}/{span_id} |
[**spans_get_io**](SpansApi.md#spans_get_io) | **GET** /v1/spans/{tenant_id}/{trace_id}/{span_id}/io |


# **spans_get**
> CanonicalSpan spans_get(tenant_id, trace_id, span_id, unmask=unmask, reason=reason, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.canonical_span import CanonicalSpan
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
    api_instance = palette_client.SpansApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    trace_id = 'trace_id_example' # str | trace_id
    span_id = 'span_id_example' # str | span_id
    unmask = True # bool |  (optional)
    reason = 'reason_example' # str |  (optional)
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.spans_get(tenant_id, trace_id, span_id, unmask=unmask, reason=reason, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of SpansApi->spans_get:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SpansApi->spans_get: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **trace_id** | **str**| trace_id |
 **span_id** | **str**| span_id |
 **unmask** | **bool**|  | [optional]
 **reason** | **str**|  | [optional]
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**CanonicalSpan**](CanonicalSpan.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Get a canonical span |  -  |
**400** | Invalid request, scope, or filter |  -  |
**401** | Missing or invalid credentials |  -  |
**403** | Credentials lack the required scope |  -  |
**404** | Resource not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **spans_get_io**
> SpanIoResponse spans_get_io(tenant_id, trace_id, span_id, unmask=unmask, reason=reason, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.span_io_response import SpanIoResponse
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
    api_instance = palette_client.SpansApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    trace_id = 'trace_id_example' # str | trace_id
    span_id = 'span_id_example' # str | span_id
    unmask = True # bool |  (optional)
    reason = 'reason_example' # str |  (optional)
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.spans_get_io(tenant_id, trace_id, span_id, unmask=unmask, reason=reason, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of SpansApi->spans_get_io:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SpansApi->spans_get_io: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **trace_id** | **str**| trace_id |
 **span_id** | **str**| span_id |
 **unmask** | **bool**|  | [optional]
 **reason** | **str**|  | [optional]
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**SpanIoResponse**](SpanIoResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Get span input/output metadata |  -  |
**400** | Invalid request, scope, or filter |  -  |
**401** | Missing or invalid credentials |  -  |
**403** | Credentials lack the required scope |  -  |
**404** | Resource not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
