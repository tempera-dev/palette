# ErrorStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | **int32** | HTTP status code corresponding to the canonical RPC status. |
**Details** | **[]map[string]interface{}** | Machine-readable standard error details. |
**Message** | **string** | Developer-facing English problem description. |
**Status** | **string** | Canonical &#x60;google.rpc.Code&#x60; enum name. |

## Methods

### NewErrorStatus

`func NewErrorStatus(code int32, details []map[string]interface{}, message string, status string, ) *ErrorStatus`

NewErrorStatus instantiates a new ErrorStatus object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewErrorStatusWithDefaults

`func NewErrorStatusWithDefaults() *ErrorStatus`

NewErrorStatusWithDefaults instantiates a new ErrorStatus object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *ErrorStatus) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *ErrorStatus) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *ErrorStatus) SetCode(v int32)`

SetCode sets Code field to given value.


### GetDetails

`func (o *ErrorStatus) GetDetails() []map[string]interface{}`

GetDetails returns the Details field if non-nil, zero value otherwise.

### GetDetailsOk

`func (o *ErrorStatus) GetDetailsOk() (*[]map[string]interface{}, bool)`

GetDetailsOk returns a tuple with the Details field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDetails

`func (o *ErrorStatus) SetDetails(v []map[string]interface{})`

SetDetails sets Details field to given value.


### GetMessage

`func (o *ErrorStatus) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *ErrorStatus) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *ErrorStatus) SetMessage(v string)`

SetMessage sets Message field to given value.


### GetStatus

`func (o *ErrorStatus) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ErrorStatus) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ErrorStatus) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
