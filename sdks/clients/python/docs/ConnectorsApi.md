# palette_client.ConnectorsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connectors_connect**](ConnectorsApi.md#connectors_connect) | **POST** /v1/connectors/{tenantId}/{projectId}/connect |
[**connectors_get_skills**](ConnectorsApi.md#connectors_get_skills) | **GET** /v1/connectors/{tenantId}/{projectId}/skills |
[**connectors_invoke_tool**](ConnectorsApi.md#connectors_invoke_tool) | **POST** /v1/connectors/{tenantId}/{projectId}/invoke |
[**connectors_list**](ConnectorsApi.md#connectors_list) | **GET** /v1/connectors/{tenantId}/{projectId} |
[**connectors_list_tools**](ConnectorsApi.md#connectors_list_tools) | **GET** /v1/connectors/{tenantId}/{projectId}/tools |
[**connectors_status**](ConnectorsApi.md#connectors_status) | **GET** /v1/connectors/{tenantId}/{projectId}/status |


# **connectors_connect**
> ConnectionLink connectors_connect(tenant_id, project_id, connect_connector_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.connect_connector_request import ConnectConnectorRequest
from palette_client.models.connection_link import ConnectionLink
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
    api_instance = palette_client.ConnectorsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    connect_connector_request = palette_client.ConnectConnectorRequest() # ConnectConnectorRequest |
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.connectors_connect(tenant_id, project_id, connect_connector_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of ConnectorsApi->connectors_connect:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ConnectorsApi->connectors_connect: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **connect_connector_request** | [**ConnectConnectorRequest**](ConnectConnectorRequest.md)|  |
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**ConnectionLink**](ConnectionLink.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | One-time login link to authorize the app |  -  |
**400** | A google.rpc.Status error envelope. |  -  |
**401** | A google.rpc.Status error envelope. |  -  |
**403** | A google.rpc.Status error envelope. |  -  |
**501** | A google.rpc.Status error envelope. |  -  |
**0** | A google.rpc.Status error envelope. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **connectors_get_skills**
> ConnectorSkillsResponse connectors_get_skills(tenant_id, project_id, toolkit, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.connector_skills_response import ConnectorSkillsResponse
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
    api_instance = palette_client.ConnectorsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    toolkit = 'toolkit_example' # str | Toolkit slug to scope the request to.
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.connectors_get_skills(tenant_id, project_id, toolkit, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of ConnectorsApi->connectors_get_skills:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ConnectorsApi->connectors_get_skills: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **toolkit** | **str**| Toolkit slug to scope the request to. |
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**ConnectorSkillsResponse**](ConnectorSkillsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Generated prompting scaffold (skill cards) for a toolkit |  -  |
**400** | A google.rpc.Status error envelope. |  -  |
**401** | A google.rpc.Status error envelope. |  -  |
**403** | A google.rpc.Status error envelope. |  -  |
**501** | A google.rpc.Status error envelope. |  -  |
**0** | A google.rpc.Status error envelope. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **connectors_invoke_tool**
> ToolExecution connectors_invoke_tool(tenant_id, project_id, invoke_connector_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.invoke_connector_request import InvokeConnectorRequest
from palette_client.models.tool_execution import ToolExecution
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
    api_instance = palette_client.ConnectorsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    invoke_connector_request = palette_client.InvokeConnectorRequest() # InvokeConnectorRequest |
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.connectors_invoke_tool(tenant_id, project_id, invoke_connector_request, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of ConnectorsApi->connectors_invoke_tool:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ConnectorsApi->connectors_invoke_tool: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **invoke_connector_request** | [**InvokeConnectorRequest**](InvokeConnectorRequest.md)|  |
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**ToolExecution**](ToolExecution.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Execute a connector tool and return its result envelope |  -  |
**400** | A google.rpc.Status error envelope. |  -  |
**401** | A google.rpc.Status error envelope. |  -  |
**403** | A google.rpc.Status error envelope. |  -  |
**501** | A google.rpc.Status error envelope. |  -  |
**0** | A google.rpc.Status error envelope. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **connectors_list**
> ConnectorListResponse connectors_list(tenant_id, project_id, page_size=page_size, page_token=page_token, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.connector_list_response import ConnectorListResponse
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
    api_instance = palette_client.ConnectorsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    page_size = 56 # int | Maximum number of apps to return. Zero selects the server default. (optional)
    page_token = 'page_token_example' # str | Opaque continuation token returned by the preceding list request. (optional)
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.connectors_list(tenant_id, project_id, page_size=page_size, page_token=page_token, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of ConnectorsApi->connectors_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ConnectorsApi->connectors_list: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **page_size** | **int**| Maximum number of apps to return. Zero selects the server default. | [optional]
 **page_token** | **str**| Opaque continuation token returned by the preceding list request. | [optional]
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**ConnectorListResponse**](ConnectorListResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | List connectable third-party apps (catalog) |  -  |
**400** | A google.rpc.Status error envelope. |  -  |
**401** | A google.rpc.Status error envelope. |  -  |
**403** | A google.rpc.Status error envelope. |  -  |
**501** | A google.rpc.Status error envelope. |  -  |
**0** | A google.rpc.Status error envelope. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **connectors_list_tools**
> ConnectorToolListResponse connectors_list_tools(tenant_id, project_id, toolkit, page_size=page_size, page_token=page_token, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.connector_tool_list_response import ConnectorToolListResponse
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
    api_instance = palette_client.ConnectorsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    toolkit = 'toolkit_example' # str | Toolkit slug to list tools for.
    page_size = 56 # int | Maximum number of tools to return. Zero selects the server default. (optional)
    page_token = 'page_token_example' # str | Opaque continuation token returned by the preceding list request. (optional)
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.connectors_list_tools(tenant_id, project_id, toolkit, page_size=page_size, page_token=page_token, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of ConnectorsApi->connectors_list_tools:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ConnectorsApi->connectors_list_tools: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **toolkit** | **str**| Toolkit slug to list tools for. |
 **page_size** | **int**| Maximum number of tools to return. Zero selects the server default. | [optional]
 **page_token** | **str**| Opaque continuation token returned by the preceding list request. | [optional]
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**ConnectorToolListResponse**](ConnectorToolListResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | List a toolkit&#39;s executable tools with input schemas |  -  |
**400** | A google.rpc.Status error envelope. |  -  |
**401** | A google.rpc.Status error envelope. |  -  |
**403** | A google.rpc.Status error envelope. |  -  |
**501** | A google.rpc.Status error envelope. |  -  |
**0** | A google.rpc.Status error envelope. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **connectors_status**
> ConnectionStatus connectors_status(tenant_id, project_id, toolkit, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)



### Example


```python
import palette_client
from palette_client.models.connection_status import ConnectionStatus
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
    api_instance = palette_client.ConnectorsApi(api_client)
    tenant_id = 'tenant_id_example' # str | tenant_id
    project_id = 'project_id_example' # str | project_id
    toolkit = 'toolkit_example' # str | Toolkit slug to scope the request to.
    authorization = 'authorization_example' # str | Bearer API token for strict auth (optional)
    x_palette_api_key = 'x_palette_api_key_example' # str | API key alternative for strict auth (optional)
    x_palette_project_id = 'x_palette_project_id_example' # str | Strict-auth project scope (optional)
    x_palette_environment_id = 'x_palette_environment_id_example' # str | Strict-auth environment scope (optional)

    try:
        api_response = api_instance.connectors_status(tenant_id, project_id, toolkit, authorization=authorization, x_palette_api_key=x_palette_api_key, x_palette_project_id=x_palette_project_id, x_palette_environment_id=x_palette_environment_id)
        print("The response of ConnectorsApi->connectors_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ConnectorsApi->connectors_status: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenant_id** | **str**| tenant_id |
 **project_id** | **str**| project_id |
 **toolkit** | **str**| Toolkit slug to scope the request to. |
 **authorization** | **str**| Bearer API token for strict auth | [optional]
 **x_palette_api_key** | **str**| API key alternative for strict auth | [optional]
 **x_palette_project_id** | **str**| Strict-auth project scope | [optional]
 **x_palette_environment_id** | **str**| Strict-auth environment scope | [optional]

### Return type

[**ConnectionStatus**](ConnectionStatus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Connection status of a toolkit for this project |  -  |
**400** | A google.rpc.Status error envelope. |  -  |
**401** | A google.rpc.Status error envelope. |  -  |
**403** | A google.rpc.Status error envelope. |  -  |
**501** | A google.rpc.Status error envelope. |  -  |
**0** | A google.rpc.Status error envelope. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
