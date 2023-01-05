package com.oppzippy.openscq30.ui.devicelistitem

import android.view.View
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView.ViewHolder
import com.oppzippy.openscq30.R

class DeviceListItemViewHolder : ViewHolder {
    val nameView: TextView
    val macAddressView: TextView

    constructor(itemView: View) : super(itemView) {
        nameView = itemView.findViewById(R.id.name)
        macAddressView = itemView.findViewById(R.id.macAddress)
    }
}
