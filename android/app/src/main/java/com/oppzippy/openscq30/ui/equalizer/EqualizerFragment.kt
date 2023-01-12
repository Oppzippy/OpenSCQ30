package com.oppzippy.openscq30.ui.equalizer

import android.os.Bundle
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.AdapterView
import android.widget.AdapterView.OnItemSelectedListener
import android.widget.ArrayAdapter
import android.widget.TextView

import androidx.databinding.DataBindingUtil
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.whenStarted
import com.google.android.material.slider.Slider
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.databinding.FragmentEqualizerBinding
import com.oppzippy.openscq30.lib.EqualizerConfiguration
import com.oppzippy.openscq30.lib.SoundcoreDeviceState
import com.oppzippy.openscq30.soundcoredevice.contentEquals
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch
import java.text.NumberFormat
import kotlin.math.roundToInt

class EqualizerFragment(private val stateFlow: StateFlow<SoundcoreDeviceState>) :
    Fragment(R.layout.fragment_equalizer) {
    private lateinit var binding: FragmentEqualizerBinding
    private lateinit var viewModel: EqualizerViewModel
    private lateinit var bandOffsets: StateFlow<ByteArray?>
    private val _equalizerConfiguration: MutableStateFlow<EqualizerConfiguration?> =
        MutableStateFlow(null)
    val equalizerConfiguration: Flow<EqualizerConfiguration> =
        _equalizerConfiguration.filterNotNull()
            .distinctUntilChanged { old, new -> old.contentEquals(new) }
    private val profiles = EqualizerProfile.values()

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?, savedInstanceState: Bundle?
    ): View {
        binding = DataBindingUtil.inflate(
            inflater, R.layout.fragment_equalizer, container, false
        )
        viewModel = ViewModelProvider(this)[EqualizerViewModel::class.java]
        binding.viewmodel = viewModel
        binding.lifecycleOwner = viewLifecycleOwner

        lifecycleScope.launch {
            stateFlow.collectLatest {
                Log.i("test", "test")
                viewModel.setBandOffsets(it.equalizerConfiguration().bandOffsets().volumeOffsets())
            }
        }

        bandOffsets = viewModel.bandOffsets


        binding.profile.adapter = ArrayAdapter(
            requireContext(),
            android.R.layout.simple_spinner_dropdown_item,
            profiles,
        )
        binding.profile.setSelection(0, false)


        binding.root.findViewById<TextView>(R.id.band100Label).text = getString(R.string.hz, 100)
        binding.root.findViewById<TextView>(R.id.band200Label).text = getString(R.string.hz, 200)
        binding.root.findViewById<TextView>(R.id.band400Label).text = getString(R.string.hz, 400)
        binding.root.findViewById<TextView>(R.id.band800Label).text = getString(R.string.hz, 800)
        binding.root.findViewById<TextView>(R.id.band1600Label).text = getString(R.string.hz, 1600)
        binding.root.findViewById<TextView>(R.id.band3200Label).text = getString(R.string.hz, 3200)
        binding.root.findViewById<TextView>(R.id.band6400Label).text = getString(R.string.hz, 6400)
        binding.root.findViewById<TextView>(R.id.band12800Label).text =
            getString(R.string.hz, 12800)

        val formatter = { value: Float ->
            val format = NumberFormat.getInstance()
            format.minimumFractionDigits = 1
            format.maximumFractionDigits = 1
            format.minimumIntegerDigits = 1
            format.format(value / 10)
        }
        getSliders().forEach {
            it.setLabelFormatter(formatter)
        }

        return binding.root
    }

    override fun onStart() {
        super.onStart()
        val indexTracker = ProfileIndexTracker(profiles)

        lifecycleScope.launch {
            bandOffsets.filterNotNull().collect {
                val matchingProfileIndex = indexTracker[it]
                binding.profile.setSelection(matchingProfileIndex)
            }
        }

        binding.profile.onItemSelectedListener = object : OnItemSelectedListener {
            override fun onItemSelected(
                parent: AdapterView<*>?, view: View?, position: Int, id: Long
            ) {
                val config =
                    profiles[position].toEqualizerConfiguration(viewModel.bandOffsets.value)
                viewModel.setBandOffsets(config.bandOffsets().volumeOffsets())
            }

            override fun onNothingSelected(parent: AdapterView<*>?) {}
        }

        getSliders().forEach { slider ->
            slider.addOnChangeListener { _, _, _ ->
                val volumeOffsets =
                    getSliders().map { it.value.roundToInt().toByte() }.toByteArray()
                val matchingProfileIndex = indexTracker[volumeOffsets]
                val equalizerConfiguration =
                    profiles[matchingProfileIndex].toEqualizerConfiguration(volumeOffsets)
                _equalizerConfiguration.value = equalizerConfiguration
            }
        }
    }

    private fun getSliders(): List<Slider> {
        return listOf(
            binding.band100,
            binding.band200,
            binding.band400,
            binding.band800,
            binding.band1600,
            binding.band3200,
            binding.band6400,
            binding.band12800,
        )
    }
}