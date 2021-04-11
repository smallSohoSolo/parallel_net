package com.parallelnet.android.net

import android.content.Context
import com.squareup.moshi.Moshi
import kotlinx.coroutines.*
import okhttp3.OkHttpClient
import retrofit2.Response
import retrofit2.Retrofit
import retrofit2.converter.moshi.MoshiConverterFactory
import java.util.*
import java.util.concurrent.TimeUnit

object BaseApi {
    private const val BASE_URL = "http://httpbin.org/"

    private fun getUrl(): String {
        return BASE_URL
    }

    val retrofitInstance: Retrofit by lazy(mode = LazyThreadSafetyMode.SYNCHRONIZED) {
        val okHttpClient = OkHttpClient.Builder()
            .connectTimeout(30L, TimeUnit.SECONDS)
            .build()
        Retrofit.Builder()
            .baseUrl(getUrl())
//            .callFactory(ParallelCallFactory())
            .addConverterFactory(
                MoshiConverterFactory.create(
                    Moshi.Builder().build()
                )
            )
            .client(okHttpClient)
            .build()
    }

    /**
     * 请求基础能力
     * 注意：没有网的回调在crash里面
     */
    fun <T> requestApi(
        requestParams: HashMap<String, String>? = null,
        block: suspend (HashMap<String, String>?) -> Response<T>,
        context: Context,
        scope: CoroutineScope = MainScope(),
        success: suspend (response: Response<T>) -> Unit = {},
        failure: suspend (response: Response<T>?) -> Unit = {},
        crash: suspend (e: Throwable) -> Unit = {},
        finally: suspend (response: Response<T>?) -> Unit = {}
    ) {
        scope.launch {
            var response: Response<T>? = null
            try {
                withContext(Dispatchers.IO) {
                    response = block(requestParams)
                }
                withContext(Dispatchers.Main) {
                    if (response?.isSuccessful == true) {
                        success(response!!)
                    } else {
                        failure(response)
                    }
                }
            } catch (e: Throwable) {
                withContext(Dispatchers.Main) {
                    crash(e)
                }
            } finally {
                withContext(Dispatchers.Main) {
                    finally(response)
                }
            }
        }
    }

}

