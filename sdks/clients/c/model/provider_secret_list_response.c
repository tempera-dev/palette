#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "provider_secret_list_response.h"



static provider_secret_list_response_t *provider_secret_list_response_create_internal(
    char *next_page_token,
    list_t *provider_secrets
    ) {
    provider_secret_list_response_t *provider_secret_list_response_local_var = malloc(sizeof(provider_secret_list_response_t));
    if (!provider_secret_list_response_local_var) {
        return NULL;
    }
    provider_secret_list_response_local_var->next_page_token = next_page_token;
    provider_secret_list_response_local_var->provider_secrets = provider_secrets;

    provider_secret_list_response_local_var->_library_owned = 1;
    return provider_secret_list_response_local_var;
}

__attribute__((deprecated)) provider_secret_list_response_t *provider_secret_list_response_create(
    char *next_page_token,
    list_t *provider_secrets
    ) {
    return provider_secret_list_response_create_internal (
        next_page_token,
        provider_secrets
        );
}

void provider_secret_list_response_free(provider_secret_list_response_t *provider_secret_list_response) {
    if(NULL == provider_secret_list_response){
        return ;
    }
    if(provider_secret_list_response->_library_owned != 1){
        fprintf(stderr, "WARNING: %s() does NOT free objects allocated by the user\n", "provider_secret_list_response_free");
        return ;
    }
    listEntry_t *listEntry;
    if (provider_secret_list_response->next_page_token) {
        free(provider_secret_list_response->next_page_token);
        provider_secret_list_response->next_page_token = NULL;
    }
    if (provider_secret_list_response->provider_secrets) {
        list_ForEach(listEntry, provider_secret_list_response->provider_secrets) {
            provider_secret_metadata_free(listEntry->data);
        }
        list_freeList(provider_secret_list_response->provider_secrets);
        provider_secret_list_response->provider_secrets = NULL;
    }
    free(provider_secret_list_response);
}

cJSON *provider_secret_list_response_convertToJSON(provider_secret_list_response_t *provider_secret_list_response) {
    cJSON *item = cJSON_CreateObject();

    // provider_secret_list_response->next_page_token
    if(provider_secret_list_response->next_page_token) {
    if(cJSON_AddStringToObject(item, "nextPageToken", provider_secret_list_response->next_page_token) == NULL) {
    goto fail; //String
    }
    }


    // provider_secret_list_response->provider_secrets
    if (!provider_secret_list_response->provider_secrets) {
        goto fail;
    }
    cJSON *provider_secrets = cJSON_AddArrayToObject(item, "providerSecrets");
    if(provider_secrets == NULL) {
    goto fail; //nonprimitive container
    }

    listEntry_t *provider_secretsListEntry;
    if (provider_secret_list_response->provider_secrets) {
    list_ForEach(provider_secretsListEntry, provider_secret_list_response->provider_secrets) {
    cJSON *itemLocal = provider_secret_metadata_convertToJSON(provider_secretsListEntry->data);
    if(itemLocal == NULL) {
    goto fail;
    }
    cJSON_AddItemToArray(provider_secrets, itemLocal);
    }
    }

    return item;
fail:
    if (item) {
        cJSON_Delete(item);
    }
    return NULL;
}

provider_secret_list_response_t *provider_secret_list_response_parseFromJSON(cJSON *provider_secret_list_responseJSON){

    provider_secret_list_response_t *provider_secret_list_response_local_var = NULL;

    // define the local list for provider_secret_list_response->provider_secrets
    list_t *provider_secretsList = NULL;

    // provider_secret_list_response->next_page_token
    cJSON *next_page_token = cJSON_GetObjectItemCaseSensitive(provider_secret_list_responseJSON, "nextPageToken");
    if (cJSON_IsNull(next_page_token)) {
        next_page_token = NULL;
    }
    if (next_page_token) {
    if(!cJSON_IsString(next_page_token) && !cJSON_IsNull(next_page_token))
    {
    goto end; //String
    }
    }

    // provider_secret_list_response->provider_secrets
    cJSON *provider_secrets = cJSON_GetObjectItemCaseSensitive(provider_secret_list_responseJSON, "providerSecrets");
    if (cJSON_IsNull(provider_secrets)) {
        provider_secrets = NULL;
    }
    if (!provider_secrets) {
        goto end;
    }


    cJSON *provider_secrets_local_nonprimitive = NULL;
    if(!cJSON_IsArray(provider_secrets)){
        goto end; //nonprimitive container
    }

    provider_secretsList = list_createList();

    cJSON_ArrayForEach(provider_secrets_local_nonprimitive,provider_secrets )
    {
        if(!cJSON_IsObject(provider_secrets_local_nonprimitive)){
            goto end;
        }
        provider_secret_metadata_t *provider_secretsItem = provider_secret_metadata_parseFromJSON(provider_secrets_local_nonprimitive);

        list_addElement(provider_secretsList, provider_secretsItem);
    }


    provider_secret_list_response_local_var = provider_secret_list_response_create_internal (
        next_page_token && !cJSON_IsNull(next_page_token) ? strdup(next_page_token->valuestring) : NULL,
        provider_secretsList
        );

    return provider_secret_list_response_local_var;
end:
    if (provider_secretsList) {
        listEntry_t *listEntry = NULL;
        list_ForEach(listEntry, provider_secretsList) {
            provider_secret_metadata_free(listEntry->data);
            listEntry->data = NULL;
        }
        list_freeList(provider_secretsList);
        provider_secretsList = NULL;
    }
    return NULL;

}
