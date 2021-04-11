package com.parallelnet.android.net

import com.squareup.moshi.Json
import com.squareup.moshi.JsonClass

@JsonClass(generateAdapter = true)
data class IpResp(
    @Json(name = "origin")
    val origin: String
)