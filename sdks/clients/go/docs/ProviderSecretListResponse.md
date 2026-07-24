# ProviderSecretListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**NextPageToken** | Pointer to **NullableString** |  | [optional]
**ProviderSecrets** | [**[]ProviderSecretMetadata**](ProviderSecretMetadata.md) |  |

## Methods

### NewProviderSecretListResponse

`func NewProviderSecretListResponse(providerSecrets []ProviderSecretMetadata, ) *ProviderSecretListResponse`

NewProviderSecretListResponse instantiates a new ProviderSecretListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewProviderSecretListResponseWithDefaults

`func NewProviderSecretListResponseWithDefaults() *ProviderSecretListResponse`

NewProviderSecretListResponseWithDefaults instantiates a new ProviderSecretListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNextPageToken

`func (o *ProviderSecretListResponse) GetNextPageToken() string`

GetNextPageToken returns the NextPageToken field if non-nil, zero value otherwise.

### GetNextPageTokenOk

`func (o *ProviderSecretListResponse) GetNextPageTokenOk() (*string, bool)`

GetNextPageTokenOk returns a tuple with the NextPageToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextPageToken

`func (o *ProviderSecretListResponse) SetNextPageToken(v string)`

SetNextPageToken sets NextPageToken field to given value.

### HasNextPageToken

`func (o *ProviderSecretListResponse) HasNextPageToken() bool`

HasNextPageToken returns a boolean if a field has been set.

### SetNextPageTokenNil

`func (o *ProviderSecretListResponse) SetNextPageTokenNil(b bool)`

 SetNextPageTokenNil sets the value for NextPageToken to be an explicit nil

### UnsetNextPageToken
`func (o *ProviderSecretListResponse) UnsetNextPageToken()`

UnsetNextPageToken ensures that no value is present for NextPageToken, not even an explicit nil
### GetProviderSecrets

`func (o *ProviderSecretListResponse) GetProviderSecrets() []ProviderSecretMetadata`

GetProviderSecrets returns the ProviderSecrets field if non-nil, zero value otherwise.

### GetProviderSecretsOk

`func (o *ProviderSecretListResponse) GetProviderSecretsOk() (*[]ProviderSecretMetadata, bool)`

GetProviderSecretsOk returns a tuple with the ProviderSecrets field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderSecrets

`func (o *ProviderSecretListResponse) SetProviderSecrets(v []ProviderSecretMetadata)`

SetProviderSecrets sets ProviderSecrets field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
