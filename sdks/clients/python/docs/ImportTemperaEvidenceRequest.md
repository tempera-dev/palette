# ImportTemperaEvidenceRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_json** | **str** | Canonical compact JSON signed by the release/decision key. The endpoint rejects equivalent but non-canonical JSON so the verified bytes are unambiguous across SDKs. |
**public_key_pem** | **str** | PEM SubjectPublicKeyInfo for the Ed25519 key whose exact byte digest is pinned inside the signed payload. |
**signature_base64** | **str** | Standard-base64 detached Ed25519 signature over &#x60;canonical_json&#x60; bytes. |

## Example

```python
from palette_client.models.import_tempera_evidence_request import ImportTemperaEvidenceRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ImportTemperaEvidenceRequest from a JSON string
import_tempera_evidence_request_instance = ImportTemperaEvidenceRequest.from_json(json)
# print the JSON string representation of the object
print(ImportTemperaEvidenceRequest.to_json())

# convert the object into a dict
import_tempera_evidence_request_dict = import_tempera_evidence_request_instance.to_dict()
# create an instance of ImportTemperaEvidenceRequest from a dict
import_tempera_evidence_request_from_dict = ImportTemperaEvidenceRequest.from_dict(import_tempera_evidence_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
