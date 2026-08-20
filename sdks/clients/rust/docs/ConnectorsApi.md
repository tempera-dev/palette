# \ConnectorsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connectors_period_connect**](ConnectorsApi.md#connectors_period_connect) | **POST** /v1/connectors/{tenantId}/{projectId}/connect |
[**connectors_period_get_skills**](ConnectorsApi.md#connectors_period_get_skills) | **GET** /v1/connectors/{tenantId}/{projectId}/skills |
[**connectors_period_invoke_tool**](ConnectorsApi.md#connectors_period_invoke_tool) | **POST** /v1/connectors/{tenantId}/{projectId}/invoke |
[**connectors_period_list**](ConnectorsApi.md#connectors_period_list) | **GET** /v1/connectors/{tenantId}/{projectId} |
[**connectors_period_list_tools**](ConnectorsApi.md#connectors_period_list_tools) | **GET** /v1/connectors/{tenantId}/{projectId}/tools |
[**connectors_period_status**](ConnectorsApi.md#connectors_period_status) | **GET** /v1/connectors/{tenantId}/{projectId}/status |



## connectors_period_connect

> models::ConnectionLink connectors_period_connect(tenant_id, project_id, connect_connector_request, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**connect_connector_request** | [**ConnectConnectorRequest**](ConnectConnectorRequest.md) |  | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::ConnectionLink**](ConnectionLink.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectors_period_get_skills

> models::ConnectorSkillsResponse connectors_period_get_skills(tenant_id, project_id, toolkit, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**toolkit** | **String** | Toolkit slug to scope the request to. | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::ConnectorSkillsResponse**](ConnectorSkillsResponse.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectors_period_invoke_tool

> models::ToolExecution connectors_period_invoke_tool(tenant_id, project_id, invoke_connector_request, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**invoke_connector_request** | [**InvokeConnectorRequest**](InvokeConnectorRequest.md) |  | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::ToolExecution**](ToolExecution.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectors_period_list

> models::ConnectorListResponse connectors_period_list(tenant_id, project_id, page_size, page_token, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**page_size** | Option<**i32**> | Maximum number of apps to return. Zero selects the server default. |  |
**page_token** | Option<**String**> | Opaque continuation token returned by the preceding list request. |  |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::ConnectorListResponse**](ConnectorListResponse.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectors_period_list_tools

> models::ConnectorToolListResponse connectors_period_list_tools(tenant_id, project_id, toolkit, page_size, page_token, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**toolkit** | **String** | Toolkit slug to list tools for. | [required] |
**page_size** | Option<**i32**> | Maximum number of tools to return. Zero selects the server default. |  |
**page_token** | Option<**String**> | Opaque continuation token returned by the preceding list request. |  |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::ConnectorToolListResponse**](ConnectorToolListResponse.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectors_period_status

> models::ConnectionStatus connectors_period_status(tenant_id, project_id, toolkit, authorization, x_palette_api_key, x_palette_project_id, x_palette_environment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | tenant_id | [required] |
**project_id** | **String** | project_id | [required] |
**toolkit** | **String** | Toolkit slug to scope the request to. | [required] |
**authorization** | Option<**String**> | Bearer API token for strict auth |  |
**x_palette_api_key** | Option<**String**> | API key alternative for strict auth |  |
**x_palette_project_id** | Option<**String**> | Strict-auth project scope |  |
**x_palette_environment_id** | Option<**String**> | Strict-auth environment scope |  |

### Return type

[**models::ConnectionStatus**](ConnectionStatus.md)

### Authorization

[tempera_oauth](../README.md#tempera_oauth), [palette_api_key](../README.md#palette_api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
