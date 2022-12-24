package com.oppzippy.openscq30.ui.equalizer

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import com.oppzippy.openscq30.databinding.FragmentEqualizerBinding

class EqualizerFragment : Fragment() {

    private var _binding: FragmentEqualizerBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
            inflater: LayoutInflater,
            container: ViewGroup?,
            savedInstanceState: Bundle?
    ): View {
        val equalizerViewModel =
                ViewModelProvider(this).get(EqualizerViewModel::class.java)

        _binding = FragmentEqualizerBinding.inflate(inflater, container, false)
        val root: View = binding.root

        return root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}