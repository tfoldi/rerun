# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/views/tensor.fbs".

from __future__ import annotations

__all__ = ["TensorView"]


from ..._baseclasses import AsComponents
from ...datatypes import EntityPathLike, Utf8Like
from .. import components as blueprint_components
from ..api import SpaceView, SpaceViewContentsLike


class TensorView(SpaceView):
    """
    **View**: A view on a tensor of any dimensionality.

    Example
    -------
    ### Use a blueprint to create a TensorView.:
    ```python
    import numpy as np
    import rerun as rr
    import rerun.blueprint as rrb

    rr.init("rerun_example_tensor", spawn=True)

    tensor = np.random.randint(0, 256, (8, 6, 3, 5), dtype=np.uint8)
    rr.log("tensor", rr.Tensor(tensor, dim_names=("width", "height", "channel", "batch")))

    blueprint = rrb.Blueprint(rrb.TensorView(origin="tensor", name="Tensor"), collapse_panels=True)
    rr.send_blueprint(blueprint)
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/tensor_view/3b452ace3cdb29ada1a613eae8e8e8e165a1d396/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/tensor_view/3b452ace3cdb29ada1a613eae8e8e8e165a1d396/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/tensor_view/3b452ace3cdb29ada1a613eae8e8e8e165a1d396/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/tensor_view/3b452ace3cdb29ada1a613eae8e8e8e165a1d396/1200w.png">
      <img src="https://static.rerun.io/tensor_view/3b452ace3cdb29ada1a613eae8e8e8e165a1d396/full.png" width="640">
    </picture>
    </center>

    """

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: SpaceViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
        visible: blueprint_components.VisibleLike | None = None,
    ) -> None:
        """
        Construct a blueprint for a new TensorView view.

        Parameters
        ----------
        origin:
            The `EntityPath` to use as the origin of this view.
            All other entities will be transformed to be displayed relative to this origin.
        contents:
            The contents of the view specified as a query expression.
            This is either a single expression, or a list of multiple expressions.
            See [rerun.blueprint.archetypes.SpaceViewContents][].
        name:
            The display name of the view.
        visible:
            Whether this view is visible.

            Defaults to true if not specified.

        """

        properties: dict[str, AsComponents] = {}
        super().__init__(
            class_identifier="Tensor",
            origin=origin,
            contents=contents,
            name=name,
            visible=visible,
            properties=properties,
        )
