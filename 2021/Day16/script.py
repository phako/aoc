#!/usr/bin/env python3
import math


# Forward declaration
class Bitstream:
    pass


class Bitstream:
    """
    Class that converts a buffer of hexadecimal data into a stream of bits.
    Using substream it is possible to create a new stream starting at the streams
    current position that is limited in size
    """
    def __init__(self, hexdata: str):
        self.data = list(hexdata.strip())
        self.bitsbuffer = []
        self.size = len(self.data) * 4
        self.parent = None

    """
    Get an integer represented by the following @bits bits from the stream
    Raises IndexError if there is not enough bits available  
    """
    def read(self, bits=1) -> int:
        if bits > self.size:
            raise IndexError

        result = ""
        if self.parent:
            result = self.parent.read(bits)
        else:
            while len(self.bitsbuffer) < bits:
                self.bitsbuffer += f'{int(self.data.pop(0), 16):04b}'

            for i in range(bits):
                result += self.bitsbuffer.pop(0)

        self.size -= bits

        return int(result, 2)

    def substream(self, size: int) -> Bitstream:
        s = Bitstream('')
        s.size = size
        s.parent = self

        return s


class Packet:
    NUMBER = 4

    def __init__(self, type : int, version: int):
        self.type = type
        self.version = version

    def version_sum(self):
        raise NotImplementedError

    def evaluate(self):
        raise NotImplementedError


# Forward declaration
def parse_packet(b: Bitstream) -> Packet:
    pass


class OperatorPacket(Packet):
    def __init__(self, version, type: int, b: Bitstream):
        super(OperatorPacket, self).__init__(type, version)
        self.sub_packets = []

        # check length type of package
        length_type = b.read(1)

        # parse all subpackets
        if length_type == 0:
            # Length type 0, we have the length of our subpackets in bits
            # So we create a substream of that length and just read until there
            # is no data left
            packet_length = b.read(15)
            s = b.substream(packet_length)
            try:
                while True:
                    self.sub_packets.append(parse_packet(s))
            except IndexError:
                pass

        if length_type == 1:
            # Length type 1, try to read the given number of packets
            packet_length = b.read(11)
            for i in range(packet_length):
                self.sub_packets.append(parse_packet(b))

    def version_sum(self):
        return sum([x.version_sum() for x in self.sub_packets], self.version)

    def evaluate(self):
        if self.type == 0:
            return sum([x.evaluate() for x in self.sub_packets])
        elif self.type == 1:
            return math.prod([x.evaluate() for x in self.sub_packets])
        elif self.type == 2:
            return min([x.evaluate() for x in self.sub_packets])
        elif self.type == 3:
            return max([x.evaluate() for x in self.sub_packets])
        elif self.type == 5:
            return 1 if self.sub_packets[0].evaluate() > self.sub_packets[1].evaluate() else 0
        elif self.type == 6:
            return 1 if self.sub_packets[0].evaluate() < self.sub_packets[1].evaluate() else 0
        elif self.type == 7:
            return 1 if self.sub_packets[0].evaluate() == self.sub_packets[1].evaluate() else 0
        else:
            raise IndexError(f'Invalid type {self.type}')


class NumberPacket(Packet):
    def __init__(self, version : int, b: Bitstream):
        super(NumberPacket, self).__init__(Packet.NUMBER, version)

        self.value = 0
        # Read chunks of 5 bits, until first bit is indicating the last chunk
        while True:
            chunk_type = b.read(1)
            part = b.read(4)
            self.value <<= 4
            self.value += part

            if chunk_type == 0:
                break

    def version_sum(self):
        return self.version

    def evaluate(self):
        return self.value


def parse_packet(b: Bitstream) -> Packet:
    version = b.read(3)
    type = b.read(3)

    # number packet
    if type == 4:
        return NumberPacket(version, b)
    else:
        # Some Operator, pass on type
        return OperatorPacket(version, type, b)


if __name__ == "__main__":
    with open('input') as f:
        b = Bitstream("D2FE28")
        p = parse_packet(b)
        print(f'Sum of versions: {p.version_sum()}')
        print(f'BITS result: {p.evaluate()}')

