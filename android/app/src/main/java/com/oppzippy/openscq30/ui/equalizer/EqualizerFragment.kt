package com.oppzippy.openscq30.ui.equalizer

import android.os.Bundle
import android.util.Log
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.TextView

import androidx.databinding.DataBindingUtil
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import com.google.android.material.slider.Slider
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.databinding.FragmentEqualizerBinding
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import java.text.NumberFormat

class EqualizerFragment : Fragment(R.layout.fragment_equalizer) {
    private lateinit var viewModel: EqualizerViewModel
    lateinit var bandOffsets: Flow<IntArray>

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        val binding: FragmentEqualizerBinding = DataBindingUtil.inflate(
            inflater,
            R.layout.fragment_equalizer,
            container,
            false
        )
        viewModel = ViewModelProvider(this)[EqualizerViewModel::class.java]
        binding.viewmodel = viewModel
        binding.lifecycleOwner = viewLifecycleOwner

        bandOffsets = viewModel.bandOffsets

        binding.root.findViewById<Button>(R.id.apply).setOnClickListener {
            Log.i("test", "apply")
        }
        binding.root.findViewById<Button>(R.id.refresh).setOnClickListener {
            Log.i("test", viewModel.bandOffsets.value.toString());
            Log.i("test", "refresh")
        }

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
        binding.root.findViewById<Slider>(R.id.band100).setLabelFormatter(formatter)
        binding.root.findViewById<Slider>(R.id.band200).setLabelFormatter(formatter)
        binding.root.findViewById<Slider>(R.id.band400).setLabelFormatter(formatter)
        binding.root.findViewById<Slider>(R.id.band800).setLabelFormatter(formatter)
        binding.root.findViewById<Slider>(R.id.band1600).setLabelFormatter(formatter)
        binding.root.findViewById<Slider>(R.id.band3200).setLabelFormatter(formatter)
        binding.root.findViewById<Slider>(R.id.band6400).setLabelFormatter(formatter)
        binding.root.findViewById<Slider>(R.id.band12800).setLabelFormatter(formatter)

        return binding.root
    }

    fun setBandOffsets(bandOffsets: IntArray) {
        viewModel.setBandOffsets(bandOffsets)
    }
}