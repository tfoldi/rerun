# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from typing import Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)
from .._converters import (
    to_np_float32,
)
from ._overrides import vec4d_native_to_pa_array  # noqa: F401

__all__ = ["Vec4D", "Vec4DArray", "Vec4DArrayLike", "Vec4DLike", "Vec4DType"]


@define
class Vec4D:
    """A vector in 4D space."""

    xyzw: npt.NDArray[np.float32] = field(converter=to_np_float32)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        return np.asarray(self.xyzw, dtype=dtype)


Vec4DLike = Union[Vec4D, npt.NDArray[Any], npt.ArrayLike, Sequence[float]]

Vec4DArrayLike = Union[
    Vec4D, Sequence[Vec4DLike], npt.NDArray[Any], npt.ArrayLike, Sequence[Sequence[float]], Sequence[float]
]


# --- Arrow support ---


class Vec4DType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.list_(pa.field("item", pa.float32(), False, {}), 4), "rerun.datatypes.Vec4D")


class Vec4DArray(BaseExtensionArray[Vec4DArrayLike]):
    _EXTENSION_NAME = "rerun.datatypes.Vec4D"
    _EXTENSION_TYPE = Vec4DType

    @staticmethod
    def _native_to_pa_array(data: Vec4DArrayLike, data_type: pa.DataType) -> pa.Array:
        return vec4d_native_to_pa_array(data, data_type)


Vec4DType._ARRAY_TYPE = Vec4DArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(Vec4DType())
