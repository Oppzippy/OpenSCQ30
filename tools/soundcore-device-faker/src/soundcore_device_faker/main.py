#!/usr/bin/env python3
import sys
import tomllib
import logging
import asyncio
import os
from pathlib import Path
from typing import Any, Callable, Dict
from bumble.core import UUID
from bumble.device import Device, DeviceConfiguration
from bumble.transport import open_transport_or_link
from bumble.rfcomm import Server, make_service_sdp_records, DLC
from bumble.avdtp import (
    AVDTP_AUDIO_MEDIA_TYPE,
    Listener,
    MediaCodecCapabilities,
)
from bumble.a2dp import (
    make_audio_sink_service_sdp_records,
    A2DP_SBC_CODEC_TYPE,
    SbcMediaCodecInformation,
)
from watchdog.events import (
    DirModifiedEvent,
    FileModifiedEvent,
    FileSystemEventHandler,
)
from watchdog.observers import Observer
from soundcoresession import SoundcoreSession


class ConfigChangedHandler(FileSystemEventHandler):
    def __init__(self, reload_config: Callable[[], None]):
        self.reload_config = reload_config

    def on_modified(self, event: DirModifiedEvent | FileModifiedEvent) -> None:
        self.reload_config()


async def main() -> None:
    if len(sys.argv) < 3:
        print(
            "Usage: ./src/device_faker/main.py <transport-spec> <device-config>\n"
            "Example: ./src/device_faker/main.py usb:0 devices/q30.toml"
        )
        return

    soundcore_session: SoundcoreSession | None = None

    config: Dict = {}

    def reload_config():
        logging.info("reloading device config")
        nonlocal config
        with open(sys.argv[2], mode="rb") as file:
            config = tomllib.load(file)
        if soundcore_session is not None:
            soundcore_session.clear_responses()
            for entry in config["responses"]:
                soundcore_session.set_response(
                    bytes(entry["command"]), bytes(entry["response"])
                )

    reload_config()

    observer = Observer()
    config_path = Path(sys.argv[2])
    observer.schedule(ConfigChangedHandler(reload_config), str(config_path.parent))
    observer.start()

    async with await open_transport_or_link(sys.argv[1]) as hci_transport:
        device = Device.from_config_with_hci(
            DeviceConfiguration(
                name=config["name"], class_of_device=2360324, keystore="JsonKeyStore"
            ),
            hci_transport.source,
            hci_transport.sink,
        )
        device.classic_enabled = True

        audio_listener = Listener.for_device(device)
        audio_listener.on("connection", on_avdtp_connection)

        rfcomm_server = Server(device)

        def refresh_responses(config: Dict, soundcore_session: SoundcoreSession):
            if soundcore_session is not None:
                for entry in config["responses"]:
                    soundcore_session.set_response(
                        bytes(entry["command"]), bytes(entry["response"])
                    )

        def on_session(session: DLC) -> None:
            nonlocal soundcore_session
            nonlocal config
            soundcore_session = SoundcoreSession(session, config.get("has_checksum", True))
            refresh_responses(config, soundcore_session)

        channel_number = rfcomm_server.listen(on_session)

        rfcomm_record_handle = 0x00010001
        a2dp_record_handle = 0x00010002
        device.sdp_service_records = {
            rfcomm_record_handle: make_service_sdp_records(
                rfcomm_record_handle, channel_number, UUID(config["rfcomm_uuid"])
            ),
            a2dp_record_handle: make_audio_sink_service_sdp_records(a2dp_record_handle),
        }

        await device.power_on()
        await device.set_discoverable(True)
        await device.set_connectable(True)

        await hci_transport.source.wait_for_termination()  # type: ignore


# -----------------------------------------------------------------------------
def codec_capabilities():
    # NOTE: this shouldn't be hardcoded, but passed on the command line instead
    return MediaCodecCapabilities(
        media_type=AVDTP_AUDIO_MEDIA_TYPE,
        media_codec_type=A2DP_SBC_CODEC_TYPE,
        media_codec_information=SbcMediaCodecInformation(
            sampling_frequency=SbcMediaCodecInformation.SamplingFrequency.SF_48000
            | SbcMediaCodecInformation.SamplingFrequency.SF_44100
            | SbcMediaCodecInformation.SamplingFrequency.SF_32000
            | SbcMediaCodecInformation.SamplingFrequency.SF_16000,
            channel_mode=SbcMediaCodecInformation.ChannelMode.MONO
            | SbcMediaCodecInformation.ChannelMode.DUAL_CHANNEL
            | SbcMediaCodecInformation.ChannelMode.STEREO
            | SbcMediaCodecInformation.ChannelMode.JOINT_STEREO,
            block_length=SbcMediaCodecInformation.BlockLength.BL_4
            | SbcMediaCodecInformation.BlockLength.BL_8
            | SbcMediaCodecInformation.BlockLength.BL_12
            | SbcMediaCodecInformation.BlockLength.BL_16,
            subbands=SbcMediaCodecInformation.Subbands.S_4
            | SbcMediaCodecInformation.Subbands.S_8,
            allocation_method=SbcMediaCodecInformation.AllocationMethod.LOUDNESS
            | SbcMediaCodecInformation.AllocationMethod.SNR,
            minimum_bitpool_value=2,
            maximum_bitpool_value=53,
        ),
    )


# -----------------------------------------------------------------------------
def on_avdtp_connection(server):
    # Add a sink endpoint to the server
    sink = server.add_sink(codec_capabilities())
    sink.on("rtp_packet", on_rtp_packet)


# -----------------------------------------------------------------------------
Context: Dict[Any, Any] = {"output": None}


def on_rtp_packet(packet):
    header = packet.payload[0]
    fragmented = header >> 7
    # start = (header >> 6) & 0x01
    # last = (header >> 5) & 0x01
    number_of_frames = header & 0x0F

    if fragmented:
        print(f"RTP: fragment {number_of_frames}")
    else:
        print(f"RTP: {number_of_frames} frames")

    Context["output"].write(packet.payload[1:])


logging.basicConfig(level=os.environ.get("LOG_LEVEL", "DEBUG").upper())
asyncio.run(main())
