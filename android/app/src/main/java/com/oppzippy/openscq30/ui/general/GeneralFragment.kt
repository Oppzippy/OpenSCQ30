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
    private lateinit var binding: FragmentGeneralBinding
    private var ignoreChanges = false
    lateinit var ambientSoundMode: Flow<AmbientSoundMode?>
    lateinit var noiseCancelingMode: Flow<NoiseCancelingMode?>

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?, savedInstanceState: Bundle?
    ): View {
        binding = DataBindingUtil.inflate(
            inflater, R.layout.fragment_general, container, false
        )
        binding.lifecycleOwner = viewLifecycleOwner

        val ambientSoundMode = MutableStateFlow<AmbientSoundMode?>(null)
        val noiseCancelingMode = MutableStateFlow<NoiseCancelingMode?>(null)
        this.ambientSoundMode = ambientSoundMode
        this.noiseCancelingMode = noiseCancelingMode

        binding.ambientSoundMode.setOnCheckedChangeListener { group, checkedId ->
            val mode = when (checkedId) {
                R.id.normal -> AmbientSoundMode.Normal
                R.id.transparency -> AmbientSoundMode.Transparency
                R.id.noiseCanceling -> AmbientSoundMode.NoiseCanceling
                else -> throw IndexOutOfBoundsException("unexpected ambient sound mode id")
            }
            if (!ignoreChanges) {
                ambientSoundMode.value = mode
            }
        }
        binding.noiseCancelingMode.setOnCheckedChangeListener { group, checkedId ->
            val mode = when (checkedId) {
                R.id.transport -> NoiseCancelingMode.Transport
                R.id.indoor -> NoiseCancelingMode.Indoor
                R.id.outdoor -> NoiseCancelingMode.Outdoor
                else -> throw IndexOutOfBoundsException("unexpected ambient sound mode id")
            }
            if (!ignoreChanges) {
                noiseCancelingMode.value = mode
            }
        }

        return binding.root
    }

    fun setAmbientSoundMode(ambientSoundMode: AmbientSoundMode) {
        ignoreChanges = true
        when (ambientSoundMode) {
            AmbientSoundMode.NoiseCanceling -> binding.noiseCanceling.isChecked = true
            AmbientSoundMode.Transparency -> binding.transparency.isChecked = true
            AmbientSoundMode.Normal -> binding.normal.isChecked = true
        }
        ignoreChanges = false
    }

    fun setNoiseCancelingMode(noiseCancelingMode: NoiseCancelingMode) {
        ignoreChanges = true
        when (noiseCancelingMode) {
            NoiseCancelingMode.Transport -> binding.transport.isChecked = true
            NoiseCancelingMode.Outdoor -> binding.outdoor.isChecked = true
            NoiseCancelingMode.Indoor -> binding.indoor.isChecked = true
        }
        ignoreChanges = false
    }
}