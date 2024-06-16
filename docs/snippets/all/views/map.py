"""Use a blueprint to show a map."""

import rerun as rr
import rerun.blueprint as rrb

rr.init("rerun_example_gps_coordinates", spawn=True)

rr.log("points", rr.GpsCoordinates([[47.6344, 19.1397, 0], [47.6334, 19.1399, 1]]))

# Create a map view to display the chart.
blueprint = rrb.Blueprint(rrb.MapView(origin="points", name="MapView"), collapse_panels=True)

rr.send_blueprint(blueprint)
