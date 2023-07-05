package com.oppzippy.openscq30.ui.deviceselection.composables

import android.text.Html
import android.text.method.LinkMovementMethod
import android.widget.TextView
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.ui.theme.OpenSCQ30Theme

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AppInfo(onBackClick: () -> Unit) {
    Scaffold(topBar = {
        TopAppBar(
            title = {
                Text(text = stringResource(id = R.string.app_name))
            },
            navigationIcon = {
                IconButton(onClick = onBackClick) {
                    Icon(
                        imageVector = Icons.Filled.ArrowBack,
                        contentDescription = stringResource(R.string.back),
                    )
                }
            },
        )
    }, content = { innerPadding ->
        Row(
            horizontalArrangement = Arrangement.Center,
            modifier = Modifier
                .padding(innerPadding)
                .fillMaxWidth()
                .padding(20.dp, 20.dp),
        ) {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                modifier = Modifier.fillMaxHeight(),
            ) {
                HtmlText(stringResource(R.string.source_code))
            }
        }
    })
}

@Composable
private fun HtmlText(text: String, modifier: Modifier = Modifier) {
    val context = LocalContext.current
    val customLinkifyTextView = remember { TextView(context) }
    val font = MaterialTheme.typography.bodyLarge
    AndroidView(modifier = modifier, factory = { customLinkifyTextView }) { textView ->
        textView.textSize = font.fontSize.value
        val html = Html.fromHtml(text, Html.FROM_HTML_MODE_COMPACT)
        textView.text = html
        textView.movementMethod = LinkMovementMethod.getInstance()
    }
}

@Preview(showBackground = true)
@Composable
private fun PreviewAppInfo() {
    OpenSCQ30Theme {
        AppInfo(onBackClick = {})
    }
}
