package com.oppzippy.openscq30.ui.devicelistitem

import android.content.Context
import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import com.oppzippy.openscq30.R

class DeviceListItemAdapter(private val context: Context, private val items: List<DeviceListItem>) :
    RecyclerView.Adapter<DeviceListItemViewHolder>() {
    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): DeviceListItemViewHolder {
        return DeviceListItemViewHolder(
            LayoutInflater.from(context).inflate(R.layout.device_list_item_view, parent, false)
        )
    }

    override fun onBindViewHolder(holder: DeviceListItemViewHolder, position: Int) {
        holder.nameView.text = items[position].name
        holder.macAddressView.text = items[position].macAddress
    }

    override fun getItemCount(): Int {
        return items.size
    }
}
