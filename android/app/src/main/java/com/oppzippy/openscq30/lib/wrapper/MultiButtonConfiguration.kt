package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.protobuf.buttonConfiguration
import com.oppzippy.openscq30.lib.protobuf.multiButtonConfiguration

data class MultiButtonConfiguration(
    val leftSingleClick: ButtonConfiguration,
    val leftDoubleClick: ButtonConfiguration,
    val leftLongPress: ButtonConfiguration,
    val rightSingleClick: ButtonConfiguration,
    val rightDoubleClick: ButtonConfiguration,
    val rightLongPress: ButtonConfiguration,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.MultiButtonConfiguration = multiButtonConfiguration {
        leftSingleClick = this@MultiButtonConfiguration.leftSingleClick.toProtobuf()
        leftDoubleClick = this@MultiButtonConfiguration.leftDoubleClick.toProtobuf()
        leftLongPress = this@MultiButtonConfiguration.leftLongPress.toProtobuf()
        rightSingleClick = this@MultiButtonConfiguration.rightSingleClick.toProtobuf()
        rightDoubleClick = this@MultiButtonConfiguration.rightDoubleClick.toProtobuf()
        rightLongPress = this@MultiButtonConfiguration.rightLongPress.toProtobuf()
    }
}

fun com.oppzippy.openscq30.lib.protobuf.MultiButtonConfiguration.toKotlin(): MultiButtonConfiguration =
    MultiButtonConfiguration(
        leftSingleClick = leftSingleClick.toKotlin(),
        leftDoubleClick = leftDoubleClick.toKotlin(),
        leftLongPress = leftLongPress.toKotlin(),
        rightSingleClick = rightSingleClick.toKotlin(),
        rightDoubleClick = rightDoubleClick.toKotlin(),
        rightLongPress = rightLongPress.toKotlin(),
    )

data class ButtonConfiguration(val isEnabled: Boolean, val action: ButtonAction) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.ButtonConfiguration = buttonConfiguration {
        isEnabled = this@ButtonConfiguration.isEnabled
        action = this@ButtonConfiguration.action.toProtobuf()
    }

    fun actionOrNull(): ButtonAction? = if (isEnabled) {
        action
    } else {
        null
    }
}

fun com.oppzippy.openscq30.lib.protobuf.ButtonConfiguration.toKotlin(): ButtonConfiguration =
    ButtonConfiguration(isEnabled, action.toKotlin())

enum class ButtonAction {
    VolumeUp,
    VolumeDown,
    PreviousSong,
    NextSong,
    AmbientSoundMode,
    VoiceAssistant,
    PlayPause,
    GameMode,
    ;

    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.ButtonAction = when (this) {
        VolumeUp -> com.oppzippy.openscq30.lib.protobuf.ButtonAction.VOLUME_UP
        VolumeDown -> com.oppzippy.openscq30.lib.protobuf.ButtonAction.VOLUME_DOWN
        PreviousSong -> com.oppzippy.openscq30.lib.protobuf.ButtonAction.PREVIOUS_SONG
        NextSong -> com.oppzippy.openscq30.lib.protobuf.ButtonAction.NEXT_SONG
        AmbientSoundMode -> com.oppzippy.openscq30.lib.protobuf.ButtonAction.AMBIENT_SOUND_MODE
        VoiceAssistant -> com.oppzippy.openscq30.lib.protobuf.ButtonAction.VOICE_ASSISTANT
        PlayPause -> com.oppzippy.openscq30.lib.protobuf.ButtonAction.PLAY_PAUSE
        GameMode -> com.oppzippy.openscq30.lib.protobuf.ButtonAction.GAME_MODE
    }

    fun toStringResource(): Int = when (this) {
        VolumeUp -> R.string.volume_up
        VolumeDown -> R.string.volume_down
        PreviousSong -> R.string.previous_song
        NextSong -> R.string.next_song
        AmbientSoundMode -> R.string.ambient_sound_mode
        VoiceAssistant -> R.string.voice_assistant
        PlayPause -> R.string.play_pause
        GameMode -> R.string.game_mode
    }
}

fun com.oppzippy.openscq30.lib.protobuf.ButtonAction.toKotlin(): ButtonAction = when (this) {
    com.oppzippy.openscq30.lib.protobuf.ButtonAction.VOLUME_UP -> ButtonAction.VolumeUp
    com.oppzippy.openscq30.lib.protobuf.ButtonAction.VOLUME_DOWN -> ButtonAction.VolumeDown
    com.oppzippy.openscq30.lib.protobuf.ButtonAction.PREVIOUS_SONG -> ButtonAction.PreviousSong
    com.oppzippy.openscq30.lib.protobuf.ButtonAction.NEXT_SONG -> ButtonAction.NextSong
    com.oppzippy.openscq30.lib.protobuf.ButtonAction.AMBIENT_SOUND_MODE -> ButtonAction.AmbientSoundMode
    com.oppzippy.openscq30.lib.protobuf.ButtonAction.VOICE_ASSISTANT -> ButtonAction.VoiceAssistant
    com.oppzippy.openscq30.lib.protobuf.ButtonAction.PLAY_PAUSE -> ButtonAction.PlayPause
    com.oppzippy.openscq30.lib.protobuf.ButtonAction.GAME_MODE -> ButtonAction.GameMode
}
