package com.oppzippy.openscq30.ui.deviceselection.composables

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@Composable
fun ConnectToDeviceCard(
    modifier: Modifier = Modifier,
    name: String,
    model: String,
    macAddress: String,
    isDemo: Boolean,
) {
    DeviceCardContainer(
        modifier,
        icon = { DeviceModelIcon(Modifier.size(50.dp), model) },
    ) {
        Text(name)
        Spacer(Modifier.height(4.dp))
        Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
            Text(macAddress)
            if (isDemo) {
                Text(text = stringResource(R.string.demo))
            }
        }
    }
}

@Composable
fun AddDeviceCard(modifier: Modifier = Modifier, name: String, model: String) {
    DeviceCardContainer(
        modifier,
        icon = { DeviceModelIcon(Modifier.size(50.dp), model) },
    ) {
        Text(name)
        Spacer(Modifier.height(4.dp))
        Text(model)
    }
}

@Composable
private fun DeviceCardContainer(
    modifier: Modifier,
    icon: @Composable (modifier: Modifier) -> Unit,
    content: @Composable () -> Unit,
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(12.dp))
            .then(modifier),
        border = BorderStroke(1.dp, MaterialTheme.colorScheme.outlineVariant),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceContainerLow,
        ),
    ) {
        Row(Modifier.height(IntrinsicSize.Min), verticalAlignment = Alignment.CenterVertically) {
            Box(
                Modifier
                    .width(80.dp)
                    .fillMaxHeight()
                    .background(MaterialTheme.colorScheme.surfaceContainerHigh),
                contentAlignment = Alignment.Center,
            ) {
                icon(Modifier.size(50.dp))
            }
            Column(Modifier.padding(16.dp)) {
                content()
            }
        }
    }
}

@Composable
private fun DeviceModelIcon(modifier: Modifier, model: String) {
    // TODO lint that checks DeviceModel rust enum and ensures all variants are listed here
    // or even better, generate a kotlin sealed class from the rust enum
    when (model) {
        "SoundcoreA3028" -> HeadphonesIcon(modifier)
        "SoundcoreA3939" -> EarbudsIcon(modifier)
        "SoundcoreA3116" -> SpeakerIcon(modifier)
        else -> HeadphonesIcon(modifier)
    }
}

@Composable
private fun HeadphonesIcon(modifier: Modifier) {
    Icon(
        modifier = modifier,
        painter = painterResource(R.drawable.headphones_24px),
        contentDescription = stringResource(R.string.headphones),
    )
}

@Composable
private fun EarbudsIcon(modifier: Modifier) {
    Icon(
        modifier = modifier,
        painter = painterResource(R.drawable.earbuds_2_24px),
        contentDescription = stringResource(R.string.earbuds),
    )
}

@Composable
private fun SpeakerIcon(modifier: Modifier) {
    Icon(
        modifier = modifier,
        painter = painterResource(R.drawable.speaker_24px),
        contentDescription = stringResource(R.string.audio_speaker),
    )
}

@Preview(showBackground = true)
@Composable
private fun PreviewDeviceCard() {
    OpenSCQ30Theme {
        ConnectToDeviceCard(
            name = "Soundcore Life Q30",
            model = "SoundcoreA3028",
            macAddress = "AC:12:2F:C8:6E:08",
            isDemo = false,
        )
    }
}
