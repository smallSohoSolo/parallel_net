package com.parallelnet.android

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.navigation.fragment.findNavController
import com.parallelnet.android.net.BaseApi
import com.parallelnet.android.net.IpApi

/**
 * A simple [Fragment] subclass as the default destination in the navigation.
 */
class ParallelRequestFragment : Fragment() {

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        // Inflate the layout for this fragment
        return inflater.inflate(R.layout.fragment_first, container, false)
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        view.findViewById<Button>(R.id.button_first).setOnClickListener {
            findNavController().navigate(R.id.action_FirstFragment_to_SecondFragment)
        }
        view.findViewById<Button>(R.id.button_raw).setOnClickListener {
            BaseApi.requestApi(block = {
                IpApi.create().getFriends()
            }, success = {
                Toast.makeText(requireContext(), it.body()?.origin, Toast.LENGTH_SHORT).show()
            }, failure = {
                Toast.makeText(requireContext(), it?.body().toString(), Toast.LENGTH_SHORT).show()
            }, crash = {
                Toast.makeText(requireContext(), it.localizedMessage, Toast.LENGTH_SHORT).show()
            }, context = requireContext())
        }
    }
}