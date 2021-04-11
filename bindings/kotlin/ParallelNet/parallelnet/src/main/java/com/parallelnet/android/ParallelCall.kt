package com.parallelnet.android

import okhttp3.Call
import okhttp3.Callback
import okhttp3.Request
import okhttp3.Response
import okio.Timeout
import java.util.*

internal class ParallelCall(private val request: Request) : Call {

    init {
        System.loadLibrary("libparallelnet")
    }

    private val parallelEngine: ParallelEngine by lazy {
        ParallelEngine(UUID.randomUUID().toString(), request, this)
    }

    override fun cancel() {
        parallelEngine.cancel()
    }

    override fun clone(): Call {
        return ParallelCall(request.newBuilder().build())
    }

    override fun enqueue(responseCallback: Callback) {
        parallelEngine.enqueue(responseCallback)
    }

    override fun execute(): Response {
        return parallelEngine.execute()
    }

    override fun isCanceled(): Boolean {
        return parallelEngine.isCanceled()
    }

    override fun isExecuted(): Boolean {
        return parallelEngine.isExecuted()
    }

    override fun request(): Request {
        return request
    }

    override fun timeout(): Timeout {
        return Timeout()
    }
}