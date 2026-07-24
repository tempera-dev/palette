# ListScenariosResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**NextPageToken** | Pointer to **NullableString** |  | [optional]
**Scenarios** | [**[]Scenario**](Scenario.md) |  |

## Methods

### NewListScenariosResponse

`func NewListScenariosResponse(scenarios []Scenario, ) *ListScenariosResponse`

NewListScenariosResponse instantiates a new ListScenariosResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListScenariosResponseWithDefaults

`func NewListScenariosResponseWithDefaults() *ListScenariosResponse`

NewListScenariosResponseWithDefaults instantiates a new ListScenariosResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNextPageToken

`func (o *ListScenariosResponse) GetNextPageToken() string`

GetNextPageToken returns the NextPageToken field if non-nil, zero value otherwise.

### GetNextPageTokenOk

`func (o *ListScenariosResponse) GetNextPageTokenOk() (*string, bool)`

GetNextPageTokenOk returns a tuple with the NextPageToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextPageToken

`func (o *ListScenariosResponse) SetNextPageToken(v string)`

SetNextPageToken sets NextPageToken field to given value.

### HasNextPageToken

`func (o *ListScenariosResponse) HasNextPageToken() bool`

HasNextPageToken returns a boolean if a field has been set.

### SetNextPageTokenNil

`func (o *ListScenariosResponse) SetNextPageTokenNil(b bool)`

 SetNextPageTokenNil sets the value for NextPageToken to be an explicit nil

### UnsetNextPageToken
`func (o *ListScenariosResponse) UnsetNextPageToken()`

UnsetNextPageToken ensures that no value is present for NextPageToken, not even an explicit nil
### GetScenarios

`func (o *ListScenariosResponse) GetScenarios() []Scenario`

GetScenarios returns the Scenarios field if non-nil, zero value otherwise.

### GetScenariosOk

`func (o *ListScenariosResponse) GetScenariosOk() (*[]Scenario, bool)`

GetScenariosOk returns a tuple with the Scenarios field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScenarios

`func (o *ListScenariosResponse) SetScenarios(v []Scenario)`

SetScenarios sets Scenarios field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
