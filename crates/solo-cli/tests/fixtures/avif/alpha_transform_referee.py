#!/usr/bin/env python3
"""Independent, test-only referee for the associated-alpha ``irot`` reference.

``SOLO-WO-118 R08`` §3.1. This program measures **two** independently produced
reference PNGs — the valid ``abc_color_irot_alpha_irot.avif`` positive and the
invalid, missing-alpha-``irot`` ``abc_color_irot_alpha_NOirot.avif`` negative —
from the already pinned Ubuntu ``avifdec``/dav1d output using the already pinned
system Pillow. It never writes to the repository, never touches the network and
never re-pins anything: the immutable R03 manifest is only read and compared.

Scope, stated honestly:

* The pinned ``avifdec`` writes the **coded** image. ``irot=1`` is a 90 degree
  anticlockwise display transform, so display facts are recovered by rotating
  the whole RGBA buffer with Pillow and hashing the result. That is an audited
  mapping, not an inference from the candidate decoder.
* The negative is measured and compared, never promoted: its refusal is a media
  policy fact recorded in the manifest, and no measurement here can accept it.
* A pass means *accurate measurement*, not that any source is V4-compatible.

Exit codes: 0 all required checks passed, 1 a checked refusal, 2 usage error.
"""

import hashlib
import json
import os
import sys

from PIL import Image

SCHEMA = "solo-avif-oracle-v1"
TOP_KEYS = {"schema", "state", "note", "tool", "fixtures", "negatives"}
POSITIVE = "abc_color_irot_alpha_irot.avif"
NEGATIVE = "abc_color_irot_alpha_NOirot.avif"
PINNED_FIELDS = (
    "png_sha256",
    "rgba_sha256",
    "alpha_min",
    "alpha_max",
    "alpha_non_opaque_samples",
    "display_width",
    "display_height",
    "rgba_display_sha256",
    "source_sha256",
)
CODED_WIDTH = 512
CODED_HEIGHT = 256
MAX_COORDINATES = 64


class Refusal(Exception):
    """A checked refusal: refused input is never silently tolerated."""


def sha256_bytes(payload):
    return hashlib.sha256(payload).hexdigest()


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_png_rgba(path, label):
    if not os.path.isfile(path):
        raise Refusal("absent %s PNG: %s" % (label, os.path.basename(path)))
    try:
        with Image.open(path) as image:
            image.load()
            if "A" not in image.getbands():
                raise Refusal("%s PNG carries no alpha channel" % label)
            rgba = image.convert("RGBA")
    except Refusal:
        raise
    except Exception as error:  # any decoder/library failure is a refusal
        raise Refusal("%s PNG is unreadable: %s" % (label, error))
    width, height = rgba.size
    if (width, height) != (CODED_WIDTH, CODED_HEIGHT):
        raise Refusal(
            "%s PNG geometry %dx%d is not the pinned coded %dx%d"
            % (label, width, height, CODED_WIDTH, CODED_HEIGHT)
        )
    return rgba.tobytes(), width, height


def alpha_facts(rgba):
    values = list(rgba[3::4])
    non_opaque = sum(1 for value in values if value != 255)
    return {
        "alpha_min": min(values),
        "alpha_max": max(values),
        "alpha_non_opaque_samples": non_opaque,
    }


def rotate_acw(rgba, width, height):
    """90 degree anticlockwise rotation of the whole RGBA buffer, alpha included."""
    image = Image.frombytes("RGBA", (width, height), rgba)
    rotated = image.transpose(Image.ROTATE_90)
    return rotated.tobytes(), rotated.size[0], rotated.size[1]


def rotate_back(rgba, width, height):
    image = Image.frombytes("RGBA", (width, height), rgba)
    return image.transpose(Image.ROTATE_270).tobytes()


def coded_to_rotated(index, width, height):
    x = index % width
    y = index // width
    return (y, height - 1 - x)


def rotate_back_check(rgba, width, height):
    """The audited rotation must be exactly invertible on the measured bytes."""
    rotated, rotated_w, rotated_h = rotate_acw(rgba, width, height)
    if (rotated_w, rotated_h) != (height, width):
        raise Refusal("rotation did not transpose the coded extents")
    if rotate_back(rotated, rotated_w, rotated_h) != rgba:
        raise Refusal("the audited rotation is not invertible on the reference bytes")
    return rotated, rotated_w, rotated_h


def compare(positive, negative, width, height):
    """Enumerate every differing sample: alpha and RGB counted separately."""
    rotated_positive, rotated_w, rotated_h = rotate_acw(positive, width, height)
    rotated_negative, rotated_w2, rotated_h2 = rotate_acw(negative, width, height)
    if (rotated_w, rotated_h) != (rotated_w2, rotated_h2):
        raise Refusal("rotated extents disagree between the two references")
    alpha = []
    rgb = 0
    for pixel in range(width * height):
        base = pixel * 4
        if positive[base:base + 3] != negative[base:base + 3]:
            rgb += 1
        if positive[base + 3] != negative[base + 3]:
            alpha.append(pixel)
    inventory = []
    for pixel in alpha[:MAX_COORDINATES]:
        x = pixel % width
        y = pixel // width
        rotated_x, rotated_y = coded_to_rotated(pixel, width, height)
        inventory.append({
            "coded_x": x,
            "coded_y": y,
            "rotated_x": rotated_x,
            "rotated_y": rotated_y,
            "positive": positive[pixel * 4 + 3],
            "negative": negative[pixel * 4 + 3],
        })
    # Cross-check the coordinate mapping against the rotated buffers themselves.
    for item in inventory:
        coded_index = (item["coded_y"] * width + item["coded_x"]) * 4 + 3
        rotated_index = (item["rotated_y"] * rotated_w + item["rotated_x"]) * 4 + 3
        if rotated_positive[rotated_index] != positive[coded_index]:
            raise Refusal("rotated coordinate mapping disagrees at %r" % (item,))
        if rotated_negative[rotated_index] != negative[coded_index]:
            raise Refusal("rotated coordinate mapping disagrees at %r" % (item,))
    return {
        "alpha_differences": len(alpha),
        "rgb_differences": rgb,
        "alpha_coordinates": inventory,
        "coordinates_truncated": len(alpha) > MAX_COORDINATES,
    }


def require_positive_pin(manifest, positive_facts):
    """Every immutable R03 positive field must match this independent measurement."""
    spec = manifest["fixtures"].get(POSITIVE)
    if not isinstance(spec, dict):
        raise Refusal("manifest has no pinned positive %s" % POSITIVE)
    if spec.get("kind") != "png":
        raise Refusal("pinned positive kind is not the measured PNG class")
    for field in PINNED_FIELDS:
        if field not in spec:
            raise Refusal("pinned positive is missing field %s" % field)
        if positive_facts.get(field) is None:
            raise Refusal("the referee did not measure pinned field %s" % field)
        if spec[field] != positive_facts[field]:
            raise Refusal(
                "pinned positive %s differs: manifest %r measured %r"
                % (field, spec[field], positive_facts[field])
            )
    return spec


def check_alias(paths):
    resolved = [os.path.realpath(path) for path in paths]
    if len(set(resolved)) != len(resolved):
        raise Refusal("a referee input aliases another; the two references must be distinct")
    for path in paths:
        if not os.path.isfile(path):
            raise Refusal("a referee input is missing: %s" % os.path.basename(path))


def measure(manifest_path, positive_avif, positive_png, negative_avif, negative_png):
    check_alias([positive_avif, positive_png, negative_avif, negative_png])
    try:
        with open(manifest_path, "r", encoding="utf-8") as handle:
            manifest = json.load(handle)
    except Exception as error:
        raise Refusal("manifest is unreadable: %s" % error)
    if not isinstance(manifest, dict) or set(manifest) != TOP_KEYS:
        raise Refusal("manifest key set is not the closed R03 schema")
    if manifest["schema"] != SCHEMA:
        raise Refusal("manifest schema is not %s" % SCHEMA)
    if manifest["state"] != "pinned":
        raise Refusal("manifest is not pinned, so no reference can be adjudicated")
    pinned_positive = manifest["fixtures"].get(POSITIVE)
    if not isinstance(pinned_positive, dict):
        raise Refusal("manifest has no pinned positive entry")
    negative_spec = manifest["negatives"].get(NEGATIVE)
    if not isinstance(negative_spec, dict) or not negative_spec.get("refusal"):
        raise Refusal("manifest records no normative refusal for %s" % NEGATIVE)

    if sha256_file(positive_avif) != pinned_positive.get("source_sha256"):
        raise Refusal("positive AVIF source digest differs from the pinned manifest")
    if sha256_file(negative_avif) != negative_spec.get("source_sha256"):
        raise Refusal("negative AVIF source digest differs from the pinned manifest")

    positive, width, height = load_png_rgba(positive_png, "positive")
    negative, neg_width, neg_height = load_png_rgba(negative_png, "negative")
    if (neg_width, neg_height) != (width, height):
        raise Refusal("the two reference PNGs disagree on coded geometry")

    rotated_positive, display_w, display_h = rotate_back_check(positive, width, height)
    positive_facts = {
        "png_sha256": sha256_file(positive_png),
        "rgba_sha256": sha256_bytes(positive),
        "display_width": display_w,
        "display_height": display_h,
        "rgba_display_sha256": sha256_bytes(rotated_positive),
        "source_sha256": sha256_file(positive_avif),
    }
    positive_facts.update(alpha_facts(positive))
    require_positive_pin(manifest, positive_facts)

    negative_facts = {
        "rgba_sha256": sha256_bytes(negative),
        "rgba_display_sha256": sha256_bytes(rotate_acw(negative, width, height)[0]),
    }
    negative_facts.update(alpha_facts(negative))
    comparison = compare(positive, negative, width, height)

    facts = {
        "schema": "solo-avif-alpha-referee-v1",
        "positive": dict(positive_facts, coded_width=width, coded_height=height),
        "negative": dict(negative_facts, coded_width=neg_width, coded_height=neg_height),
        "same_coded_rgba": positive_facts["rgba_sha256"] == negative_facts["rgba_sha256"],
        "same_display_rgba": (
            positive_facts["rgba_display_sha256"] == negative_facts["rgba_display_sha256"]
        ),
        "alpha_differences": comparison["alpha_differences"],
        "rgb_differences": comparison["rgb_differences"],
        "alpha_coordinates": comparison["alpha_coordinates"],
        "coordinates_truncated": comparison["coordinates_truncated"],
        "negative_refusal": negative_spec["refusal"],
    }
    return manifest, facts


def mutation_rejections(manifest, facts):
    """Two deliberate in-memory mutations of the copied manifest must be refused."""
    rejected = 0
    mutated_source = json.loads(json.dumps(manifest))
    mutated_source["fixtures"][POSITIVE]["source_sha256"] = "0" * 64
    try:
        require_positive_pin(mutated_source, facts["positive"])
    except Refusal:
        rejected += 1
    mutated_display = json.loads(json.dumps(manifest))
    mutated_display["fixtures"][POSITIVE]["rgba_display_sha256"] = "0" * 64
    try:
        require_positive_pin(mutated_display, facts["positive"])
    except Refusal:
        rejected += 1
    return rejected


def main(argv):
    if len(argv) != 6:
        sys.stderr.write(
            "usage: alpha_transform_referee.py <manifest> <positive.avif> <positive.png> "
            "<negative.avif> <negative.png>\n"
        )
        return 2
    manifest_path, positive_avif, positive_png, negative_avif, negative_png = argv[1:]
    try:
        manifest, facts = measure(
            manifest_path, positive_avif, positive_png, negative_avif, negative_png
        )
    except Refusal as refusal:
        sys.stdout.write("n123_reference refused=%s\n" % refusal)
        return 1

    positive = facts["positive"]
    negative = facts["negative"]
    sys.stdout.write(
        "n123_reference positive_pin=matched png_sha256=%s rgba_sha256=%s "
        "rgba_display_sha256=%s alpha_min=%d alpha_max=%d alpha_non_opaque=%d "
        "coded=%dx%d display=%dx%d\n"
        % (positive["png_sha256"], positive["rgba_sha256"],
           positive["rgba_display_sha256"], positive["alpha_min"], positive["alpha_max"],
           positive["alpha_non_opaque_samples"], positive["coded_width"],
           positive["coded_height"], positive["display_width"], positive["display_height"])
    )
    sys.stdout.write(
        "n123_reference negative_measured=true rgba_sha256=%s rgba_display_sha256=%s "
        "alpha_min=%d alpha_max=%d alpha_non_opaque=%d coded=%dx%d\n"
        % (negative["rgba_sha256"], negative["rgba_display_sha256"], negative["alpha_min"],
           negative["alpha_max"], negative["alpha_non_opaque_samples"],
           negative["coded_width"], negative["coded_height"])
    )
    coordinates = ";".join(
        "%d,%d:%d>%d" % (item["coded_x"], item["coded_y"], item["positive"], item["negative"])
        for item in facts["alpha_coordinates"]
    ) or "none"
    sys.stdout.write(
        "n123_reference same_coded_rgba=%s same_display_rgba=%s alpha_differences=%d "
        "rgb_differences=%d coordinates=%s truncated=%s\n"
        % (str(facts["same_coded_rgba"]).lower(), str(facts["same_display_rgba"]).lower(),
           facts["alpha_differences"], facts["rgb_differences"], coordinates,
           str(facts["coordinates_truncated"]).lower())
    )

    rejected = mutation_rejections(manifest, facts)
    if rejected != 2:
        sys.stdout.write("n123_reference mutation_rejections=%d/2\n" % rejected)
        return 1
    sys.stdout.write("n123_reference mutation_rejections=2/2\n")
    sys.stdout.write(
        "n123_reference json=%s\n"
        % json.dumps(facts, sort_keys=True, separators=(",", ":"))
    )
    sys.stdout.write("n123_reference cleanup=complete\n")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
