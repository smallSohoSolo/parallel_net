package com.parallelnet.android

import okhttp3.Call
import okhttp3.Request

class ParallelCallFactory : Call.Factory {
    override fun newCall(request: Request): Call {
        return ParallelCall(request)
    }
}