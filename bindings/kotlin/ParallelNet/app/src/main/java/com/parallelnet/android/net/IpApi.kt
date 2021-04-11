package com.parallelnet.android.net

import retrofit2.Response
import retrofit2.http.GET

interface IpApi {

    @GET("ip")
    suspend fun getFriends(): Response<IpResp>

    companion object {
        fun create(): IpApi {
            return BaseApi.retrofitInstance.create(IpApi::class.java)
        }
    }
}