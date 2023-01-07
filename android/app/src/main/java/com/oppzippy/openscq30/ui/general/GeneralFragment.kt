package com.oppzippy.openscq30.ui.general

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.databinding.DataBindingUtil
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.databinding.FragmentGeneralBinding
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import kotlinx.coroutines.flow.*

class GeneralFragment : Fragment(R.layout.fragment_general) {
    private lateinit var viewModel: GeneralViewModel
    lateinit var ambientSoundMode: StateFlow<AmbientSoundMode>
    lateinit var noiseCancelingMode: StateFlow<NoiseCancelingMode>

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        val binding: FragmentGeneralBinding = DataBindingUtil.inflate(
            inflater,
            R.layout.fragment_general,
            container,
            false
        )
        viewModel = ViewModelProvider(this)[GeneralViewModel::class.java]
        binding.viewmodel = viewModel
        binding.lifecycleOwner = viewLifecycleOwner

        ambientSoundMode = viewModel.ambientSoundMode
        noiseCancelingMode = viewModel.noiseCancelingMode

        return binding.root
    }

    fun setAmbientSoundMode(ambientSoundMode: AmbientSoundMode) {
        viewModel.setAmbientSoundMode(ambientSoundMode)
    }

    fun setNoiseCancelingMode(noiseCancelingMode: NoiseCancelingMode) {
        viewModel.setNoiseCancelingMode(noiseCancelingMode)
    }
}