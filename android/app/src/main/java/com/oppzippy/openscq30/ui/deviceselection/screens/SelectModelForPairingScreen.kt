package com.oppzippy.openscq30.ui.deviceselection.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SearchBarDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.bindings.deviceModels
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.ui.deviceselection.components.AddDeviceModelCard

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SelectModelForPairingScreen(onModelSelected: (String) -> Unit, onBackClick: () -> Unit) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(text = stringResource(id = R.string.select_device_model))
                },
                navigationIcon = {
                    IconButton(onClick = onBackClick) {
                        Icon(
                            painter = painterResource(R.drawable.arrow_back_24px),
                            contentDescription = stringResource(R.string.back),
                        )
                    }
                },
            )
        },
        content = { innerPadding ->
            Column(
                modifier = Modifier
                    .padding(innerPadding)
                    .fillMaxSize(),
            ) {
                var searchQuery by remember { mutableStateOf("") }
                Box(Modifier.padding(horizontal = 16.dp, vertical = 4.dp)) {
                    SearchBarDefaults.InputField(
                        modifier = Modifier.fillMaxWidth(),
                        query = searchQuery,
                        onQueryChange = { searchQuery = it },
                        onSearch = {},
                        expanded = true,
                        onExpandedChange = {},
                        placeholder = { Text(stringResource(R.string.search)) },
                        leadingIcon = {
                            IconButton(onClick = onBackClick) {
                                Icon(
                                    painter = painterResource(R.drawable.search_24px),
                                    contentDescription = stringResource(R.string.back),
                                )
                            }
                        },
                    )
                }
                LazyColumn(
                    modifier = Modifier
                        .testTag("modelList")
                        .padding(horizontal = 16.dp, vertical = 12.dp)
                        .fillMaxSize(),
                    verticalArrangement = Arrangement.spacedBy(16.dp),
                ) {
                    val filteredDeviceModels = deviceModels()
                        .map { Pair(it, translateDeviceModel(it)) }
                        .filter { (model, name) ->
                            model.contains(searchQuery, true) || name.contains(searchQuery, true)
                        }
                    if (filteredDeviceModels.isNotEmpty()) {
                        items(filteredDeviceModels) { (model, name) ->
                            AddDeviceModelCard(
                                modifier = Modifier.clickable { onModelSelected(model) },
                                name = name,
                                model = model,
                            )
                        }
                    } else {
                        item {
                            Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                                Text(stringResource(R.string.no_items_found))
                            }
                        }
                    }
                }
            }
        },
    )
}
