# ConnectorToolListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**NextPageToken** | Pointer to **NullableString** |  | [optional]
**Tools** | [**[]ConnectorTool**](ConnectorTool.md) |  |

## Methods

### NewConnectorToolListResponse

`func NewConnectorToolListResponse(tools []ConnectorTool, ) *ConnectorToolListResponse`

NewConnectorToolListResponse instantiates a new ConnectorToolListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewConnectorToolListResponseWithDefaults

`func NewConnectorToolListResponseWithDefaults() *ConnectorToolListResponse`

NewConnectorToolListResponseWithDefaults instantiates a new ConnectorToolListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNextPageToken

`func (o *ConnectorToolListResponse) GetNextPageToken() string`

GetNextPageToken returns the NextPageToken field if non-nil, zero value otherwise.

### GetNextPageTokenOk

`func (o *ConnectorToolListResponse) GetNextPageTokenOk() (*string, bool)`

GetNextPageTokenOk returns a tuple with the NextPageToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextPageToken

`func (o *ConnectorToolListResponse) SetNextPageToken(v string)`

SetNextPageToken sets NextPageToken field to given value.

### HasNextPageToken

`func (o *ConnectorToolListResponse) HasNextPageToken() bool`

HasNextPageToken returns a boolean if a field has been set.

### SetNextPageTokenNil

`func (o *ConnectorToolListResponse) SetNextPageTokenNil(b bool)`

 SetNextPageTokenNil sets the value for NextPageToken to be an explicit nil

### UnsetNextPageToken
`func (o *ConnectorToolListResponse) UnsetNextPageToken()`

UnsetNextPageToken ensures that no value is present for NextPageToken, not even an explicit nil
### GetTools

`func (o *ConnectorToolListResponse) GetTools() []ConnectorTool`

GetTools returns the Tools field if non-nil, zero value otherwise.

### GetToolsOk

`func (o *ConnectorToolListResponse) GetToolsOk() (*[]ConnectorTool, bool)`

GetToolsOk returns a tuple with the Tools field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTools

`func (o *ConnectorToolListResponse) SetTools(v []ConnectorTool)`

SetTools sets Tools field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
