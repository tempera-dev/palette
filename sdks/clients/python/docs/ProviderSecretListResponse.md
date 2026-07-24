# ProviderSecretListResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**next_page_token** | **str** |  | [optional]
**provider_secrets** | [**List[ProviderSecretMetadata]**](ProviderSecretMetadata.md) |  |

## Example

```python
from palette_client.models.provider_secret_list_response import ProviderSecretListResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ProviderSecretListResponse from a JSON string
provider_secret_list_response_instance = ProviderSecretListResponse.from_json(json)
# print the JSON string representation of the object
print(ProviderSecretListResponse.to_json())

# convert the object into a dict
provider_secret_list_response_dict = provider_secret_list_response_instance.to_dict()
# create an instance of ProviderSecretListResponse from a dict
provider_secret_list_response_from_dict = ProviderSecretListResponse.from_dict(provider_secret_list_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
