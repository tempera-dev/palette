# ConnectorListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**NextPageToken** | Pointer to **NullableString** |  | [optional]
**Toolkits** | [**[]Toolkit**](Toolkit.md) |  |

## Methods

### NewConnectorListResponse

`func NewConnectorListResponse(toolkits []Toolkit, ) *ConnectorListResponse`

NewConnectorListResponse instantiates a new ConnectorListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewConnectorListResponseWithDefaults

`func NewConnectorListResponseWithDefaults() *ConnectorListResponse`

NewConnectorListResponseWithDefaults instantiates a new ConnectorListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNextPageToken

`func (o *ConnectorListResponse) GetNextPageToken() string`

GetNextPageToken returns the NextPageToken field if non-nil, zero value otherwise.

### GetNextPageTokenOk

`func (o *ConnectorListResponse) GetNextPageTokenOk() (*string, bool)`

GetNextPageTokenOk returns a tuple with the NextPageToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextPageToken

`func (o *ConnectorListResponse) SetNextPageToken(v string)`

SetNextPageToken sets NextPageToken field to given value.

### HasNextPageToken

`func (o *ConnectorListResponse) HasNextPageToken() bool`

HasNextPageToken returns a boolean if a field has been set.

### SetNextPageTokenNil

`func (o *ConnectorListResponse) SetNextPageTokenNil(b bool)`

 SetNextPageTokenNil sets the value for NextPageToken to be an explicit nil

### UnsetNextPageToken
`func (o *ConnectorListResponse) UnsetNextPageToken()`

UnsetNextPageToken ensures that no value is present for NextPageToken, not even an explicit nil
### GetToolkits

`func (o *ConnectorListResponse) GetToolkits() []Toolkit`

GetToolkits returns the Toolkits field if non-nil, zero value otherwise.

### GetToolkitsOk

`func (o *ConnectorListResponse) GetToolkitsOk() (*[]Toolkit, bool)`

GetToolkitsOk returns a tuple with the Toolkits field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetToolkits

`func (o *ConnectorListResponse) SetToolkits(v []Toolkit)`

SetToolkits sets Toolkits field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
