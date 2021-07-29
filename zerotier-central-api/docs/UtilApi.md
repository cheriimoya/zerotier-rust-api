# \UtilApi

All URIs are relative to *https://my.zerotier.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_random_token**](UtilApi.md#get_random_token) | **GET** /randomToken | Get a random 32 character token
[**get_status**](UtilApi.md#get_status) | **GET** /status | Obtain the overall status of the account tied to the API token in use.



## get_random_token

> crate::models::RandomToken get_random_token()
Get a random 32 character token

Get a random 32 character.  Used by the web UI to generate API keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RandomToken**](RandomToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status

> crate::models::Status get_status()
Obtain the overall status of the account tied to the API token in use.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

