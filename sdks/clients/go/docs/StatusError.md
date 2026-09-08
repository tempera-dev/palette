# StatusError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | **int32** | The HTTP status code, repeated in the body so the envelope is self-describing. |
**Details** | Pointer to **[]map[string]interface{}** | Structured error details. Present but empty when there are none. | [optional]
**Message** | **string** | A developer-facing description that is safe to log and safe to show. |
**RequestId** | Pointer to **string** | The server-assigned request identifier, mirrored from the x-request-id response header. | [optional]
**Status** | **string** | The canonical google.rpc.Code name, for example INVALID_ARGUMENT. |

## Methods

### NewStatusError

`func NewStatusError(code int32, message string, status string, ) *StatusError`

NewStatusError instantiates a new StatusError object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewStatusErrorWithDefaults

`func NewStatusErrorWithDefaults() *StatusError`

NewStatusErrorWithDefaults instantiates a new StatusError object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *StatusError) GetCode() int32`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *StatusError) GetCodeOk() (*int32, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *StatusError) SetCode(v int32)`

SetCode sets Code field to given value.


### GetDetails

`func (o *StatusError) GetDetails() []map[string]interface{}`

GetDetails returns the Details field if non-nil, zero value otherwise.

### GetDetailsOk

`func (o *StatusError) GetDetailsOk() (*[]map[string]interface{}, bool)`

GetDetailsOk returns a tuple with the Details field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDetails

`func (o *StatusError) SetDetails(v []map[string]interface{})`

SetDetails sets Details field to given value.

### HasDetails

`func (o *StatusError) HasDetails() bool`

HasDetails returns a boolean if a field has been set.

### GetMessage

`func (o *StatusError) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *StatusError) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *StatusError) SetMessage(v string)`

SetMessage sets Message field to given value.


### GetRequestId

`func (o *StatusError) GetRequestId() string`

GetRequestId returns the RequestId field if non-nil, zero value otherwise.

### GetRequestIdOk

`func (o *StatusError) GetRequestIdOk() (*string, bool)`

GetRequestIdOk returns a tuple with the RequestId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequestId

`func (o *StatusError) SetRequestId(v string)`

SetRequestId sets RequestId field to given value.

### HasRequestId

`func (o *StatusError) HasRequestId() bool`

HasRequestId returns a boolean if a field has been set.

### GetStatus

`func (o *StatusError) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *StatusError) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *StatusError) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
