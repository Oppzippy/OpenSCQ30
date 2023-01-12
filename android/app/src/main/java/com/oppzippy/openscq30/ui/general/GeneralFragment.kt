package com.oppzippy.openscq30.ui.general

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.databinding.DataBindingUtil
import androidx.fragment.app.Fragment
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.databinding.FragmentGeneralBinding
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch

class GeneralFragment(private val stateFlow: StateFlow<SoundcoreDeviceState>) : Fragment(R.layout.fragment_general) {
    private lateinit var binding: FragmentGeneralBinding
    private val mutableSoundMode =
        MutableStateFlow<Pair<AmbientSoundMode, NoiseCancelingMode>?>(null)
    val soundMode: Flow<Pair<AmbientSoundMode, NoiseCancelingMode>> =
        mutableSoundMode.filterNotNull()

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?, savedInstanceState: Bundle?
    ): View {
        binding = DataBindingUtil.inflate(
            inflater, R.layout.fragment_general, container, false
        )
        binding.lifecycleOwner = viewLifecycleOwner

        binding.ambientSoundMode.setOnCheckedChangeListener { _, _ ->
            val ambientSoundMode = getAmbientSoundMode()
            val noiseCancelingMode = getNoiseCancelingMode()
            if (ambientSoundMode != null && noiseCancelingMode != null) {
                mutableSoundMode.value = Pair(ambientSoundMode, noiseCancelingMode)
            } else {
                mutableSoundMode.value = null
            }
        }
        binding.noiseCancelingMode.setOnCheckedChangeListener { _, _ ->
            val ambientSoundMode = getAmbientSoundMode()
            val noiseCancelingMode = getNoiseCancelingMode()
            if (ambientSoundMode != null && noiseCancelingMode != null) {
                mutableSoundMode.value = Pair(ambientSoundMode, noiseCancelingMode)
            } else {
                mutableSoundMode.value = null
            }
        }

        lifecycleScope.launch {
            stateFlow.collectLatest {
                setAmbientSoundMode(it.ambientSoundMode())
                setNoiseCancelingMode(it.noiseCancelingMode())
            }
        }

        return binding.root
    }

    private fun getNoiseCancelingMode(): NoiseCancelingMode? {
        return when (binding.noiseCancelingMode.checkedRadioButtonId) {
            R.id.transport -> NoiseCancelingMode.Transport
            R.id.indoor -> NoiseCancelingMode.Indoor
            R.id.outdoor -> NoiseCancelingMode.Outdoor
            else -> null
        }
    }

    private fun getAmbientSoundMode(): AmbientSoundMode? {
        return when (binding.ambientSoundMode.checkedRadioButtonId) {
            R.id.normal -> AmbientSoundMode.Normal
            R.id.transparency -> AmbientSoundMode.Transparency
            R.id.noiseCanceling -> AmbientSoundMode.NoiseCanceling
            else -> null
        }
    }

    fun setAmbientSoundMode(ambientSoundMode: AmbientSoundMode) {
        lifecycleScope.launchWhenStarted {
            when (ambientSoundMode) {
                AmbientSoundMode.NoiseCanceling -> binding.noiseCanceling.isChecked = true
                AmbientSoundMode.Transparency -> binding.transparency.isChecked = true
                AmbientSoundMode.Normal -> binding.normal.isChecked = true
            }
        }
    }

    fun setNoiseCancelingMode(noiseCancelingMode: NoiseCancelingMode) {
        lifecycleScope.launchWhenStarted {
            when (noiseCancelingMode) {
                NoiseCancelingMode.Transport -> binding.transport.isChecked = true
                NoiseCancelingMode.Outdoor -> binding.outdoor.isChecked = true
                NoiseCancelingMode.Indoor -> binding.indoor.isChecked = true
            }
        }
    }
}