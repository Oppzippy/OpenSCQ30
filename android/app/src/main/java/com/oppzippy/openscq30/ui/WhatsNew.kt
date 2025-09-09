package com.oppzippy.openscq30.ui

import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.TextLinkStyles
import androidx.compose.ui.text.fromHtml
import androidx.compose.ui.text.style.TextDecoration
import com.oppzippy.openscq30.R

@Composable
fun WhatsNew(messageHtml: String, onClose: () -> Unit) {
    AlertDialog(
        onDismissRequest = onClose,
        confirmButton = { Button(onClick = onClose) { Text(stringResource(R.string.close)) } },
        text = {
            Text(
                AnnotatedString.fromHtml(
                    messageHtml,
                    TextLinkStyles(SpanStyle(textDecoration = TextDecoration.Underline)),
                ),
            )
        },
    )
}
