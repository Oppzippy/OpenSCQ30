from bumble.rfcomm import DLC
import logging


class SoundcoreSession:
    def __init__(self, rfcomm_session: DLC, has_checksum: bool):
        self.__rfcomm_session = rfcomm_session
        self.__rfcomm_session.sink = self.on_packet_received
        self.__responses = {}
        self.__has_checksum = has_checksum

    def on_packet_received(self, packet: bytes) -> None:
        logging.info(f"<-- {list(packet)}")
        command = packet[5:7]
        if response := self.__responses.get(command):
            packet = self.__format_outbound_packet(command, response)
            self.__rfcomm_session.write(packet)
            logging.info(f"--> {list(packet)}sent known response")
        else:
            packet = self.__format_outbound_packet(command, b"")
            self.__rfcomm_session.write(packet)
            logging.warning(f"--> {list(packet)}no known response, sent ack")

    def set_response(self, command: bytes, response: bytes):
        self.__responses[command] = response

    def clear_responses(self):
        self.__responses.clear()

    def __format_outbound_packet(self, command: bytes, data: bytes) -> bytes:
        packet_length = len(data) + (10 if self.__has_checksum else 9)
        packet = bytes(
            [
                0x09,
                0xFF,
                0x00,
                0x00,
                0x01,
                *command,
                *packet_length.to_bytes(2, "little"),
                *data,
            ]
        )
        if self.__has_checksum:
            packet += (sum(packet) % 256).to_bytes()
        return packet
