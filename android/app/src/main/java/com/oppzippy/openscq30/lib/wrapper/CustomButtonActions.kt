package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.protobuf.buttonState
import com.oppzippy.openscq30.lib.protobuf.customButtonActions

data class CustomButtonActions(
    val leftSingleClick: ButtonState,
    val leftDoubleClick: ButtonState,
    val leftLongPress: ButtonState,
    val rightSingleClick: ButtonState,
    val rightDoubleClick: ButtonState,
    val rightLongPress: ButtonState,
) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.CustomButtonActions = customButtonActions {
        leftSingleClick = this@CustomButtonActions.leftSingleClick.toProtobuf()
        leftDoubleClick = this@CustomButtonActions.leftDoubleClick.toProtobuf()
        leftLongPress = this@CustomButtonActions.leftLongPress.toProtobuf()
        rightSingleClick = this@CustomButtonActions.rightSingleClick.toProtobuf()
        rightDoubleClick = this@CustomButtonActions.rightDoubleClick.toProtobuf()
        rightLongPress = this@CustomButtonActions.rightLongPress.toProtobuf()
    }
}

fun com.oppzippy.openscq30.lib.protobuf.CustomButtonActions.toKotlin(): CustomButtonActions = CustomButtonActions(
    leftSingleClick = leftSingleClick.toKotlin(),
    leftDoubleClick = leftDoubleClick.toKotlin(),
    leftLongPress = leftLongPress.toKotlin(),
    rightSingleClick = rightSingleClick.toKotlin(),
    rightDoubleClick = rightDoubleClick.toKotlin(),
    rightLongPress = rightLongPress.toKotlin(),
)

data class ButtonState(val isEnabled: Boolean, val action: ButtonAction) {
    fun toProtobuf(): com.oppzippy.openscq30.lib.protobuf.ButtonState = buttonState {
        isEnabled = this@ButtonState.isEnabled
        action = this@ButtonState.action.toProtobuf()
    }

    fun actionOrNull(): ButtonAction? = if (isEnabled) {
        action
    } else {
        null
    }
}

fun com.oppzippy.openscq30.lib.protobuf.ButtonState.toKotlin(): ButtonState = ButtonState(isEnabled, action.toKotlin())

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
