from __future__ import annotations

from typing import TYPE_CHECKING, Any

from ..components import ViewCoordinates as Component

if TYPE_CHECKING:
    from . import ViewCoordinates


class ViewCoordinatesExt:
    # <BEGIN_GENERATED:declarations>
    # This section is generated by running `scripts/generate_view_coordinate_defs.py --python`
    # The following declarations are replaced in `deferred_patch_class`.
    ULF: ViewCoordinates = None  # type: ignore[assignment]
    UFL: ViewCoordinates = None  # type: ignore[assignment]
    LUF: ViewCoordinates = None  # type: ignore[assignment]
    LFU: ViewCoordinates = None  # type: ignore[assignment]
    FUL: ViewCoordinates = None  # type: ignore[assignment]
    FLU: ViewCoordinates = None  # type: ignore[assignment]
    ULB: ViewCoordinates = None  # type: ignore[assignment]
    UBL: ViewCoordinates = None  # type: ignore[assignment]
    LUB: ViewCoordinates = None  # type: ignore[assignment]
    LBU: ViewCoordinates = None  # type: ignore[assignment]
    BUL: ViewCoordinates = None  # type: ignore[assignment]
    BLU: ViewCoordinates = None  # type: ignore[assignment]
    URF: ViewCoordinates = None  # type: ignore[assignment]
    UFR: ViewCoordinates = None  # type: ignore[assignment]
    RUF: ViewCoordinates = None  # type: ignore[assignment]
    RFU: ViewCoordinates = None  # type: ignore[assignment]
    FUR: ViewCoordinates = None  # type: ignore[assignment]
    FRU: ViewCoordinates = None  # type: ignore[assignment]
    URB: ViewCoordinates = None  # type: ignore[assignment]
    UBR: ViewCoordinates = None  # type: ignore[assignment]
    RUB: ViewCoordinates = None  # type: ignore[assignment]
    RBU: ViewCoordinates = None  # type: ignore[assignment]
    BUR: ViewCoordinates = None  # type: ignore[assignment]
    BRU: ViewCoordinates = None  # type: ignore[assignment]
    DLF: ViewCoordinates = None  # type: ignore[assignment]
    DFL: ViewCoordinates = None  # type: ignore[assignment]
    LDF: ViewCoordinates = None  # type: ignore[assignment]
    LFD: ViewCoordinates = None  # type: ignore[assignment]
    FDL: ViewCoordinates = None  # type: ignore[assignment]
    FLD: ViewCoordinates = None  # type: ignore[assignment]
    DLB: ViewCoordinates = None  # type: ignore[assignment]
    DBL: ViewCoordinates = None  # type: ignore[assignment]
    LDB: ViewCoordinates = None  # type: ignore[assignment]
    LBD: ViewCoordinates = None  # type: ignore[assignment]
    BDL: ViewCoordinates = None  # type: ignore[assignment]
    BLD: ViewCoordinates = None  # type: ignore[assignment]
    DRF: ViewCoordinates = None  # type: ignore[assignment]
    DFR: ViewCoordinates = None  # type: ignore[assignment]
    RDF: ViewCoordinates = None  # type: ignore[assignment]
    RFD: ViewCoordinates = None  # type: ignore[assignment]
    FDR: ViewCoordinates = None  # type: ignore[assignment]
    FRD: ViewCoordinates = None  # type: ignore[assignment]
    DRB: ViewCoordinates = None  # type: ignore[assignment]
    DBR: ViewCoordinates = None  # type: ignore[assignment]
    RDB: ViewCoordinates = None  # type: ignore[assignment]
    RBD: ViewCoordinates = None  # type: ignore[assignment]
    BDR: ViewCoordinates = None  # type: ignore[assignment]
    BRD: ViewCoordinates = None  # type: ignore[assignment]
    RIGHT_HAND_X_UP: ViewCoordinates = None  # type: ignore[assignment]
    RIGHT_HAND_X_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    RIGHT_HAND_Y_UP: ViewCoordinates = None  # type: ignore[assignment]
    RIGHT_HAND_Y_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    RIGHT_HAND_Z_UP: ViewCoordinates = None  # type: ignore[assignment]
    RIGHT_HAND_Z_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    LEFT_HAND_X_UP: ViewCoordinates = None  # type: ignore[assignment]
    LEFT_HAND_X_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    LEFT_HAND_Y_UP: ViewCoordinates = None  # type: ignore[assignment]
    LEFT_HAND_Y_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    LEFT_HAND_Z_UP: ViewCoordinates = None  # type: ignore[assignment]
    LEFT_HAND_Z_DOWN: ViewCoordinates = None  # type: ignore[assignment]
    # <END_GENERATED:declarations>

    @staticmethod
    def deferred_patch_class(cls: Any) -> None:
        # <BEGIN_GENERATED:definitions>
        # This section is generated by running `scripts/generate_view_coordinate_defs.py --python`
        cls.ULF = cls(Component.ULF)
        cls.UFL = cls(Component.UFL)
        cls.LUF = cls(Component.LUF)
        cls.LFU = cls(Component.LFU)
        cls.FUL = cls(Component.FUL)
        cls.FLU = cls(Component.FLU)
        cls.ULB = cls(Component.ULB)
        cls.UBL = cls(Component.UBL)
        cls.LUB = cls(Component.LUB)
        cls.LBU = cls(Component.LBU)
        cls.BUL = cls(Component.BUL)
        cls.BLU = cls(Component.BLU)
        cls.URF = cls(Component.URF)
        cls.UFR = cls(Component.UFR)
        cls.RUF = cls(Component.RUF)
        cls.RFU = cls(Component.RFU)
        cls.FUR = cls(Component.FUR)
        cls.FRU = cls(Component.FRU)
        cls.URB = cls(Component.URB)
        cls.UBR = cls(Component.UBR)
        cls.RUB = cls(Component.RUB)
        cls.RBU = cls(Component.RBU)
        cls.BUR = cls(Component.BUR)
        cls.BRU = cls(Component.BRU)
        cls.DLF = cls(Component.DLF)
        cls.DFL = cls(Component.DFL)
        cls.LDF = cls(Component.LDF)
        cls.LFD = cls(Component.LFD)
        cls.FDL = cls(Component.FDL)
        cls.FLD = cls(Component.FLD)
        cls.DLB = cls(Component.DLB)
        cls.DBL = cls(Component.DBL)
        cls.LDB = cls(Component.LDB)
        cls.LBD = cls(Component.LBD)
        cls.BDL = cls(Component.BDL)
        cls.BLD = cls(Component.BLD)
        cls.DRF = cls(Component.DRF)
        cls.DFR = cls(Component.DFR)
        cls.RDF = cls(Component.RDF)
        cls.RFD = cls(Component.RFD)
        cls.FDR = cls(Component.FDR)
        cls.FRD = cls(Component.FRD)
        cls.DRB = cls(Component.DRB)
        cls.DBR = cls(Component.DBR)
        cls.RDB = cls(Component.RDB)
        cls.RBD = cls(Component.RBD)
        cls.BDR = cls(Component.BDR)
        cls.BRD = cls(Component.BRD)
        cls.RIGHT_HAND_X_UP = cls(Component.RIGHT_HAND_X_UP)
        cls.RIGHT_HAND_X_DOWN = cls(Component.RIGHT_HAND_X_DOWN)
        cls.RIGHT_HAND_Y_UP = cls(Component.RIGHT_HAND_Y_UP)
        cls.RIGHT_HAND_Y_DOWN = cls(Component.RIGHT_HAND_Y_DOWN)
        cls.RIGHT_HAND_Z_UP = cls(Component.RIGHT_HAND_Z_UP)
        cls.RIGHT_HAND_Z_DOWN = cls(Component.RIGHT_HAND_Z_DOWN)
        cls.LEFT_HAND_X_UP = cls(Component.LEFT_HAND_X_UP)
        cls.LEFT_HAND_X_DOWN = cls(Component.LEFT_HAND_X_DOWN)
        cls.LEFT_HAND_Y_UP = cls(Component.LEFT_HAND_Y_UP)
        cls.LEFT_HAND_Y_DOWN = cls(Component.LEFT_HAND_Y_DOWN)
        cls.LEFT_HAND_Z_UP = cls(Component.LEFT_HAND_Z_UP)
        cls.LEFT_HAND_Z_DOWN = cls(Component.LEFT_HAND_Z_DOWN)
        # <END_GENERATED:definitions>