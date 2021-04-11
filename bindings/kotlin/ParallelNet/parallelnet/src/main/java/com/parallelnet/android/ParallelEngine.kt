package com.parallelnet.android

import okhttp3.Call
import okhttp3.Callback
import okhttp3.Request
import okhttp3.Response
import java.io.IOException

class ParallelEngine(private val id: String, private val request: Request, private val call: Call) {

    private lateinit var responseCallback: Callback

    external fun cancel()

    external fun isCanceled(): Boolean

    external fun isExecuted(): Boolean

    private external fun nativeEnqueue()

    private external fun nativeExecute(): String

    fun enqueue(responseCallback: Callback) {
        this.responseCallback = responseCallback
        nativeEnqueue()
    }

    private fun enqueueSuccess(string: String) {
        responseCallback.onResponse(call, Response.Builder().build())
    }

    private fun enqueueFail(string: String) {
        responseCallback.onFailure(call, IOException(string))
    }

    fun execute(): Response {
        nativeExecute()
        return Response.Builder().build()
    }

}