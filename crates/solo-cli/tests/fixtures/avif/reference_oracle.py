#!/usr/bin/env python3
"""Independent, test-only AVIF reference oracle (SOLO-WO-118 R03).

Standard library only, except the closed ``png`` command, which imports Pillow.
Nothing here is a product dependency and nothing here may be imported by a crate.

Closed commands::

    y4m <path>   parse exactly one YUV4MPEG2 frame and hash its exact pixel planes
    png <path>   parse one reference PNG with Pillow and hash decoded RGBA bytes
    self-test    prove the parser on synthetic input and prove each refusal

Aggregate framing (documented, deliberately *not* the product's own scheme):
for each plane in Y, U, V order feed ``b"PLANE|" + ordinal + b"|" + name +
b"|" + stride + b"|" + rows + b"|" + length + b"\n"`` and then that plane's raw
sample bytes into one SHA-256 stream.  The framing is unambiguous because every
field is ASCII decimal or a fixed plane name between pipe delimiters.
"""

import hashlib
import json
import os
import sys
import tempfile

MAX_FILE_BYTES = 64 * 1024 * 1024
MAX_DIMENSION = 8192
MAX_PIXELS = 32_000_000

# chroma token -> (coded depth, horizontal subsampling, vertical subsampling)
CHROMA_SPECS = {
    "420": (8, 2, 2),
    "420p10": (10, 2, 2),
    "420p12": (12, 2, 2),
    "444": (8, 1, 1),
    "444p10": (10, 1, 1),
    "444p12": (12, 1, 1),
}

ALLOWED_HEADER_TAGS = ("W", "H", "F", "I", "A", "C", "X")

USAGE = "usage: reference_oracle.py {y4m <path>|png <path>|self-test}"


class Refusal(Exception):
    """A checked refusal: the input is not an acceptable reference measurement."""


def sha256_hex(chunks):
    digest = hashlib.sha256()
    for chunk in chunks:
        digest.update(chunk)
    return digest.hexdigest()


def read_bounded(path):
    """Read a declared-size file, refusing empty and oversized inputs."""
    try:
        size = os.stat(path).st_size
    except OSError as error:
        raise Refusal("unreadable input: %s" % error)
    if size == 0:
        raise Refusal("empty input")
    if size > MAX_FILE_BYTES:
        raise Refusal("input exceeds the checked %d byte bound" % MAX_FILE_BYTES)
    with open(path, "rb") as handle:
        data = handle.read(MAX_FILE_BYTES + 1)
    if len(data) != size:
        raise Refusal("input changed length while being read")
    return data


def parse_decimal(text, what):
    if not text or not text.isdigit():
        raise Refusal("%s is not an unsigned decimal" % what)
    return int(text)


def parse_header(line):
    if not line.startswith(b"YUV4MPEG2"):
        raise Refusal("missing YUV4MPEG2 signature")
    tags = line.split(b" ")
    if tags[0] != b"YUV4MPEG2":
        raise Refusal("malformed YUV4MPEG2 signature token")
    width = height = interlace = None
    chroma = None
    for tag in tags[1:]:
        if not tag:
            raise Refusal("empty header tag")
        letter = chr(tag[0])
        value = tag[1:]
        if letter not in ALLOWED_HEADER_TAGS:
            raise Refusal("unknown header tag %r" % letter)
        if letter == "W":
            width = parse_decimal(value, "width")
        elif letter == "H":
            height = parse_decimal(value, "height")
        elif letter == "C":
            chroma = value.decode("ascii", "strict")
        elif letter == "F":
            if b":" not in value:
                raise Refusal("malformed frame-rate tag")
        elif letter == "I":
            interlace = value
        elif letter == "A":
            if b":" not in value:
                raise Refusal("malformed aspect-ratio tag")
    if width is None or height is None or chroma is None:
        raise Refusal("header must declare W, H and C")
    if not 1 <= width <= MAX_DIMENSION or not 1 <= height <= MAX_DIMENSION:
        raise Refusal("dimension outside 1..%d" % MAX_DIMENSION)
    if width * height > MAX_PIXELS:
        raise Refusal("pixel product above the checked bound")
    if chroma not in CHROMA_SPECS:
        raise Refusal("unsupported chroma token %r" % chroma)
    if interlace is not None and interlace != b"p":
        raise Refusal("interlaced reference output is refused")
    return width, height, chroma


def plane_geometry(width, height, chroma):
    depth, sub_x, sub_y = CHROMA_SPECS[chroma]
    sample_bytes = 1 if depth == 8 else 2
    chroma_width = width if sub_x == 1 else (width + 1) // 2
    chroma_height = height if sub_y == 1 else (height + 1) // 2
    layout = (
        ("Y", width, height),
        ("U", chroma_width, chroma_height),
        ("V", chroma_width, chroma_height),
    )
    planes = []
    for ordinal, (name, columns, rows) in enumerate(layout):
        stride = columns * sample_bytes
        length = stride * rows
        planes.append(
            {
                "ordinal": ordinal,
                "name": name,
                "columns": columns,
                "rows": rows,
                "sample_bytes": sample_bytes,
                "stride": stride,
                "length": length,
            }
        )
    return depth, planes


def check_sample_range(plane_bytes, depth):
    if depth == 8:
        return
    limit = (1 << depth) - 1
    for index in range(0, len(plane_bytes), 2):
        sample = plane_bytes[index] | (plane_bytes[index + 1] << 8)
        if sample > limit:
            raise Refusal("sample %d exceeds coded depth %d" % (sample, depth))

def measure_y4m(path):
    """Measure exactly one YUV4MPEG2 frame; refuse anything else."""
    raw = read_bounded(path)
    newline = raw.find(b"\n")
    if newline < 0:
        raise Refusal("missing header terminator")
    width, height, chroma = parse_header(raw[:newline])
    depth, planes = plane_geometry(width, height, chroma)
    body = raw[newline + 1 :]
    frame_end = body.find(b"\n")
    if frame_end < 0:
        raise Refusal("missing FRAME record")
    if not body[:frame_end].startswith(b"FRAME"):
        raise Refusal("missing FRAME marker")
    payload = body[frame_end + 1 :]
    expected = 0
    for plane in planes:
        expected += plane["length"]
    if len(payload) != expected:
        raise Refusal(
            "payload is %d bytes but the checked geometry requires exactly %d"
            % (len(payload), expected)
        )

    records = []
    aggregate = hashlib.sha256()
    offset = 0
    for plane in planes:
        chunk = payload[offset : offset + plane["length"]]
        offset += plane["length"]
        check_sample_range(chunk, depth)
        framing = "PLANE|%d|%s|%d|%d|%d\n" % (
            plane["ordinal"],
            plane["name"],
            plane["stride"],
            plane["rows"],
            plane["length"],
        )
        aggregate.update(framing.encode("ascii"))
        aggregate.update(chunk)
        records.append(
            {
                "ordinal": plane["ordinal"],
                "name": plane["name"],
                "columns": plane["columns"],
                "rows": plane["rows"],
                "stride": plane["stride"],
                "length": plane["length"],
                "sha256": sha256_hex([chunk]),
            }
        )

    return {
        "fixture": os.path.basename(path),
        "source_sha256": sha256_hex([raw]),
        "width": width,
        "height": height,
        "chroma": chroma,
        "depth": depth,
        "planes": records,
        "aggregate_sha256": aggregate.hexdigest(),
    }


def measure_png(path):
    """Measure decoded RGBA bytes of a reference PNG; Pillow is confined here."""
    try:
        from PIL import Image
    except ImportError:
        raise Refusal("Pillow is required for the png command")

    raw = read_bounded(path)
    try:
        image = Image.open(path)
    except Exception as error:  # Pillow raises a family of decode errors
        raise Refusal("Pillow refused the PNG: %s" % error)
    width, height = image.size
    if not 1 <= width <= MAX_DIMENSION or not 1 <= height <= MAX_DIMENSION:
        raise Refusal("PNG dimension outside the checked bound")
    rgba = image.convert("RGBA").tobytes()
    if len(rgba) != width * height * 4:
        raise Refusal("decoded RGBA length does not match the reported geometry")
    alpha = rgba[3::4]
    return {
        "fixture": os.path.basename(path),
        "png_sha256": sha256_hex([raw]),
        "width": width,
        "height": height,
        "rgba_sha256": sha256_hex([rgba]),
        "alpha_min": min(alpha),
        "alpha_max": max(alpha),
        "alpha_non_opaque_samples": sum(1 for value in alpha if value != 255),
    }


def emit(payload):
    sys.stdout.write(json.dumps(payload, sort_keys=True, separators=(",", ":")))
    sys.stdout.write("\n")


def main(argv):
    if len(argv) < 2:
        sys.stderr.write(USAGE + "\n")
        return 64
    command = argv[1]
    if command == "self-test":
        if len(argv) != 2:
            sys.stderr.write(USAGE + "\n")
            return 64
        return run_self_test()
    if command in ("y4m", "png"):
        if len(argv) != 3:
            sys.stderr.write(USAGE + "\n")
            return 64
        try:
            payload = measure_y4m(argv[2]) if command == "y4m" else measure_png(argv[2])
        except Refusal as refusal:
            sys.stderr.write("refused: %s\n" % refusal)
            return 2
        emit(payload)
        return 0
    sys.stderr.write(USAGE + "\n")
    return 64


def _write_temp(directory, name, data):
    path = os.path.join(directory, name)
    with open(path, "wb") as handle:
        handle.write(data)
    return path


def run_self_test():
    """Prove the parser on synthetic geometry and prove every refusal."""
    checks = []

    def expect(condition, label):
        checks.append((bool(condition), label))

    def expect_refusal(data, label):
        with tempfile.TemporaryDirectory() as directory:
            path = _write_temp(directory, "case.y4m", data)
            try:
                measure_y4m(path)
            except Refusal:
                expect(True, label)
            else:
                expect(False, label)

    # 4:4:4, 8-bit, 2x1. Geometry is stated here by hand so the expectation
    # does not reuse plane_geometry().
    y_plane = bytes([0x01, 0x02])
    u_plane = bytes([0x03, 0x04])
    v_plane = bytes([0x05, 0x06])
    body = b"YUV4MPEG2 W2 H1 F1:1 Ip A1:1 C444\nFRAME\n" + y_plane + u_plane + v_plane
    with tempfile.TemporaryDirectory() as directory:
        result = measure_y4m(_write_temp(directory, "abc444.y4m", body))
    expect(result["width"] == 2 and result["height"] == 1, "444 width/height")
    expect(result["chroma"] == "444" and result["depth"] == 8, "444 chroma/depth")
    expect(result["planes"][0]["sha256"] == hashlib.sha256(y_plane).hexdigest(), "444 Y digest")
    expect(result["planes"][1]["sha256"] == hashlib.sha256(u_plane).hexdigest(), "444 U digest")
    expect(result["planes"][2]["sha256"] == hashlib.sha256(v_plane).hexdigest(), "444 V digest")
    expect(result["planes"][0]["stride"] == 2 and result["planes"][0]["length"] == 2, "444 Y stride")
    expected_aggregate = hashlib.sha256(
        b"PLANE|0|Y|2|1|2\n"
        + y_plane
        + b"PLANE|1|U|2|1|2\n"
        + u_plane
        + b"PLANE|2|V|2|1|2\n"
        + v_plane
    ).hexdigest()
    expect(result["aggregate_sha256"] == expected_aggregate, "444 aggregate framing")

    # 4:2:0, 10-bit, 2x2: Y is 2 columns x 2 rows of little-endian 16-bit samples,
    # chroma is ceil(2/2) x ceil(2/2) = 1x1 per plane.
    y10 = b"".join(value.to_bytes(2, "little") for value in (0, 1, 2, 1023))
    u10 = (7).to_bytes(2, "little")
    v10 = (9).to_bytes(2, "little")
    body10 = b"YUV4MPEG2 W2 H2 F1:1 Ip A1:1 C420p10\nFRAME\n" + y10 + u10 + v10
    with tempfile.TemporaryDirectory() as directory:
        result10 = measure_y4m(_write_temp(directory, "abc420p10.y4m", body10))
    expect(result10["depth"] == 10 and result10["chroma"] == "420p10", "420p10 depth")
    expect(result10["planes"][0]["stride"] == 4 and result10["planes"][0]["rows"] == 2, "420p10 Y geometry")
    expect(result10["planes"][1]["columns"] == 1 and result10["planes"][1]["length"] == 2, "420p10 chroma geometry")
    expect(result10["planes"][0]["sha256"] == hashlib.sha256(y10).hexdigest(), "420p10 Y digest")
    expected10 = hashlib.sha256(
        b"PLANE|0|Y|4|2|8\n"
        + y10
        + b"PLANE|1|U|2|1|2\n"
        + u10
        + b"PLANE|2|V|2|1|2\n"
        + v10
    ).hexdigest()
    expect(result10["aggregate_sha256"] == expected10, "420p10 aggregate framing")

    # Refusals: each must be a checked Refusal, never a silent acceptance.
    expect_refusal(b"YUV4MPEG2 W2 H1 C444\nFRAME\n" + y_plane + u_plane + v_plane + body, "two frames refused")
    expect_refusal(b"YUV4MPEG2 W2 H1 C444\nFRAME\n" + y_plane + u_plane + v_plane + b"\x00", "trailing byte refused")
    expect_refusal(b"YUV4MPEG2 W2 H1 C444\nWRONG\n" + y_plane + u_plane + v_plane, "missing FRAME refused")
    expect_refusal(b"YUV4MPEG2 W2 H1 Q1 C444\nFRAME\n" + y_plane + u_plane + v_plane, "unknown tag refused")
    expect_refusal(b"YUV4MPEG2 W2 H1 C422\nFRAME\n" + y_plane + u_plane + v_plane, "unknown chroma refused")
    expect_refusal(b"YUV4MPEG2 W0 H1 C444\nFRAME\n", "zero width refused")
    expect_refusal(b"YUV4MPEG2 W8200 H1 C444\nFRAME\n", "overbound width refused")
    expect_refusal(b"", "empty input refused")
    oversample = b"".join(value.to_bytes(2, "little") for value in (0, 0, 0, 1024))
    expect_refusal(
        b"YUV4MPEG2 W2 H2 C420p10\nFRAME\n" + oversample + u10 + v10,
        "sample above coded depth refused",
    )

    failed = [label for ok, label in checks if not ok]
    for ok, label in checks:
        sys.stdout.write(("PASS " if ok else "FAIL ") + label + "\n")
    sys.stdout.write("self-test: %d passed, %d failed\n" % (len(checks) - len(failed), len(failed)))
    return 0 if not failed else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))

