# NativeIngestRequestCost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AmountMicros** | **int64** |  |
**Currency** | [**Currency**](Currency.md) |  |

## Methods

### NewNativeIngestRequestCost

`func NewNativeIngestRequestCost(amountMicros int64, currency Currency, ) *NativeIngestRequestCost`

NewNativeIngestRequestCost instantiates a new NativeIngestRequestCost object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewNativeIngestRequestCostWithDefaults

`func NewNativeIngestRequestCostWithDefaults() *NativeIngestRequestCost`

NewNativeIngestRequestCostWithDefaults instantiates a new NativeIngestRequestCost object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAmountMicros

`func (o *NativeIngestRequestCost) GetAmountMicros() int64`

GetAmountMicros returns the AmountMicros field if non-nil, zero value otherwise.

### GetAmountMicrosOk

`func (o *NativeIngestRequestCost) GetAmountMicrosOk() (*int64, bool)`

GetAmountMicrosOk returns a tuple with the AmountMicros field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAmountMicros

`func (o *NativeIngestRequestCost) SetAmountMicros(v int64)`

SetAmountMicros sets AmountMicros field to given value.


### GetCurrency

`func (o *NativeIngestRequestCost) GetCurrency() Currency`

GetCurrency returns the Currency field if non-nil, zero value otherwise.

### GetCurrencyOk

`func (o *NativeIngestRequestCost) GetCurrencyOk() (*Currency, bool)`

GetCurrencyOk returns a tuple with the Currency field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrency

`func (o *NativeIngestRequestCost) SetCurrency(v Currency)`

SetCurrency sets Currency field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
