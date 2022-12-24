package com.oppzippy.openscq30.ui.general

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import com.oppzippy.openscq30.databinding.FragmentGeneralBinding
import com.oppzippy.openscq30.lib.HelloWorldRust

class GeneralFragment : Fragment() {

    private var _binding: FragmentGeneralBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
            inflater: LayoutInflater,
            container: ViewGroup?,
            savedInstanceState: Bundle?
    ): View {
        val generalViewModel =
                ViewModelProvider(this).get(GeneralViewModel::class.java)

        _binding = FragmentGeneralBinding.inflate(inflater, container, false)
        val root: View = binding.root

        val r = HelloWorldRust()
        val textView: TextView = binding.textView
        generalViewModel.text.observe(viewLifecycleOwner) {
            textView.text = r.greet("test")
        }
        textView.text = r.greet("test")
        return root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}