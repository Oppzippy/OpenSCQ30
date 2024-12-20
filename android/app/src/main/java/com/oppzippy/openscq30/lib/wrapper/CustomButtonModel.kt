package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.protobuf.customButtonModel
import com.oppzippy.openscq30.lib.protobuf.noTwsButtonAction
import com.oppzippy.openscq30.lib.protobuf.twsButtonAction

data class CustomButtonModel(
    val leftSingleClick: NoTwsButtonAction,
    val leftDoubleClick: TwsButtonAction,
    val leftLongPress: TwsButtonAction,
    val rightSingleClick: NoTwsButtonAction,
    val rightDoubleClick: TwsButtonAction,
    val rightLongPress: TwsButtonAction,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.CustomButtonModel = customButtonModel {
        leftSingleClick = this@CustomButtonModel.leftSingleClick.toProtobuf()
        leftDoubleClick = this@CustomButtonModel.leftDoubleClick.toProtobuf()
        leftLongPress = this@CustomButtonModel.leftLongPress.toProtobuf()
        rightSingleClick = this@CustomButtonModel.rightSingleClick.toProtobuf()
        rightDoubleClick = this@CustomButtonModel.rightDoubleClick.toProtobuf()
        rightLongPress = this@CustomButtonModel.rightLongPress.toProtobuf()
    }
}

fun com.oppzippy.openscq30.lib.protobuf.CustomButtonModel.toKotlin(): CustomButtonModel = CustomButtonModel(
    leftSingleClick = leftSingleClick.toKotlin(),
    leftDoubleClick = leftDoubleClick.toKotlin(),
    leftLongPress = leftLongPress.toKotlin(),
    rightSingleClick = rightSingleClick.toKotlin(),
    rightDoubleClick = rightDoubleClick.toKotlin(),
    rightLongPress = rightLongPress.toKotlin(),
)

data class TwsButtonAction(
    val isEnabled: Boolean,
    val twsConnectedAction: ButtonAction,
    val twsDisconnectedAction: ButtonAction,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.TwsButtonAction = twsButtonAction {
        isEnabled = this@TwsButtonAction.isEnabled
        twsConnectedAction = this@TwsButtonAction.twsDisconnectedAction.toProtobuf()
        twsDisconnectedAction = this@TwsButtonAction.twsDisconnectedAction.toProtobuf()
    }

    fun connectedActionOrNull(): ButtonAction? = if (isEnabled) {
        twsConnectedAction
    } else {
        null
    }
}

fun com.oppzippy.openscq30.lib.protobuf.TwsButtonAction.toKotlin(): TwsButtonAction = TwsButtonAction(
    isEnabled = isEnabled,
    twsConnectedAction = twsConnectedAction.toKotlin(),
    twsDisconnectedAction = twsDisconnectedAction.toKotlin(),
)

data class NoTwsButtonAction(val isEnabled: Boolean, val action: ButtonAction) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.NoTwsButtonAction = noTwsButtonAction {
        isEnabled = this@NoTwsButtonAction.isEnabled
        action = this@NoTwsButtonAction.action.toProtobuf()
    }

    fun actionOrNull(): ButtonAction? = if (isEnabled) {
        action
    } else {
        null
    }
}

fun com.oppzippy.openscq30.lib.protobuf.NoTwsButtonAction.toKotlin(): NoTwsButtonAction =
    NoTwsButtonAction(isEnabled, action.toKotlin())

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
