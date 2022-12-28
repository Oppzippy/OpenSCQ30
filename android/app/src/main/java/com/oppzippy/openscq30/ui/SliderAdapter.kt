package com.oppzippy.openscq30.ui

import androidx.databinding.BindingAdapter
import androidx.databinding.InverseBindingAdapter
import androidx.databinding.InverseBindingListener
import com.google.android.material.slider.Slider

@InverseBindingAdapter(attribute = "android:value")
fun getSliderValue(slider: Slider): Float {
    return slider.value
}

@BindingAdapter("android:valueAttrChanged")
fun setSliderListeners(slider: Slider, attrChange: InverseBindingListener) {
    val listener = Slider.OnChangeListener { _, _, _ -> attrChange.onChange() }
    slider.addOnChangeListener(listener)
}
