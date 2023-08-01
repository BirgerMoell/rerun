# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)

__all__ = ["Arrows3D"]


@define(str=False, repr=False)
class Arrows3D(Archetype):
    """
    A batch of 3D arrows with optional colors, radii, labels, etc.

    Example
    -------
    ```python
    from math import tau

    import numpy as np
    import rerun as rr
    import rerun.experimental as rr2
    from rerun.experimental import dt as rrd

    rr.init("arrow", spawn=True)

    lengths = np.log2(np.arange(0, 100) + 1)
    angles = np.arange(start=0, stop=tau, step=tau * 0.01)
    vectors = np.column_stack([np.sin(angles) * lengths, np.zeros(100), np.cos(angles) * lengths])
    arrows = [rrd.Arrow3D(origin=[0, 0, 0], vector=v) for v in vectors]
    colors = [[1.0 - c, c, 0.5, 0.5] for c in angles / tau]

    rr2.log("arrows", rr2.Arrows3D(arrows, colors=colors))
    ```
    """

    arrows: components.Arrow3DArray = field(
        metadata={"component": "primary"},
        converter=components.Arrow3DArray.from_similar,  # type: ignore[misc]
    )
    """
    All the individual arrows that make up the batch.
    """

    radii: components.RadiusArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.RadiusArray.from_similar,  # type: ignore[misc]
    )
    """
    Optional radii for the arrows.

    The shaft is rendered as a cylinder with `radius = 0.5 * radius`.
    The tip is rendered as a cone with `height = 2.0 * radius` and `radius = 1.0 * radius`.
    """

    colors: components.ColorArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.ColorArray.from_similar,  # type: ignore[misc]
    )
    """
    Optional colors for the points.

    The colors are interpreted as RGB or RGBA in sRGB gamma-space,
    As either 0-1 floats or 0-255 integers, with separate alpha.
    """

    labels: components.LabelArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.LabelArray.from_similar,  # type: ignore[misc]
    )
    """
    Optional text labels for the arrows.
    """

    class_ids: components.ClassIdArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.ClassIdArray.from_similar,  # type: ignore[misc]
    )
    """
    Optional class Ids for the points.

    The class ID provides colors and labels if not specified explicitly.
    """

    instance_keys: components.InstanceKeyArray | None = field(
        metadata={"component": "secondary"},
        default=None,
        converter=components.InstanceKeyArray.from_similar,  # type: ignore[misc]
    )
    """
    Unique identifiers for each individual point in the batch.
    """

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
