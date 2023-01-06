package com.oppzippy.openscq30.ui.devicelistitem

import android.view.View
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView.ViewHolder
import com.oppzippy.openscq30.R

class DeviceListItemViewHolder(itemView: View) : ViewHolder(itemView) {
    val nameView: TextView = itemView.findViewById(R.id.name)
    val macAddressView: TextView = itemView.findViewById(R.id.macAddress)

    fun setOnClickListener(listener: View.OnClickListener) {
        itemView.setOnClickListener(listener)
    }
}
