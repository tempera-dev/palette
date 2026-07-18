#include <stdlib.h>
#include <stdio.h>
#include "../include/apiClient.h"
#include "../include/list.h"
#include "../external/cJSON.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "../model/create_review_queue_http_request.h"
#include "../model/dataset_case.h"
#include "../model/enqueue_review_task_from_trace_http_request.h"
#include "../model/error_response.h"
#include "../model/promote_review_annotation_http_request.h"
#include "../model/review_annotation.h"
#include "../model/review_queue.h"
#include "../model/review_task.h"
#include "../model/review_task_state.h"
#include "../model/submit_review_annotation_http_request.h"

// Enum  for ReviewsAPI_reviewsListTasks
typedef enum  { palette_api_reviewsListTasks__NULL = 0, palette_api_reviewsListTasks__open, palette_api_reviewsListTasks__submitted, palette_api_reviewsListTasks__cancelled } palette_api_reviewsListTasks_state_e;


review_queue_t*
ReviewsAPI_reviewsCreateQueue(apiClient_t *apiClient, char *tenant_id, char *project_id, create_review_queue_http_request_t *create_review_queue_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


review_task_t*
ReviewsAPI_reviewsEnqueueTaskFromTrace(apiClient_t *apiClient, char *tenant_id, char *project_id, char *queue_id, enqueue_review_task_from_trace_http_request_t *enqueue_review_task_from_trace_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


list_t*
ReviewsAPI_reviewsListTasks(apiClient_t *apiClient, char *tenant_id, char *project_id, char *queue_id, review_task_state_e state, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


dataset_case_t*
ReviewsAPI_reviewsPromoteAnnotation(apiClient_t *apiClient, char *tenant_id, char *project_id, char *queue_id, char *task_id, char *annotation_id, promote_review_annotation_http_request_t *promote_review_annotation_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


review_annotation_t*
ReviewsAPI_reviewsSubmitAnnotation(apiClient_t *apiClient, char *tenant_id, char *project_id, char *queue_id, char *task_id, submit_review_annotation_http_request_t *submit_review_annotation_http_request, char *authorization, char *x_palette_api_key, char *x_palette_project_id, char *x_palette_environment_id);


