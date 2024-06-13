// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/gps_coordinates.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../components/color.hpp"
#include "../components/position3d.hpp"
#include "../components/radius.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: Set of `GPS coordinates`
    ///
    /// ## Example
    ///
    /// ### GPS coordinates
    /// ![image](https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_gps_coordinates");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     rec.log("points", rerun::GpsCoordinates({{47.6343f, 19.1397f, 0.0f}, {47.6344f, 19.1395f, 1.0f}}));
    /// }
    /// ```
    struct GpsCoordinates {
        /// All the GpsCoordinates
        Collection<rerun::components::Position3D> coordinates;

        /// Optional radii for the points, effectively turning them into circles.
        std::optional<Collection<rerun::components::Radius>> radii;

        /// Optional colors for the points.
        std::optional<Collection<rerun::components::Color>> colors;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.GpsCoordinatesIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        GpsCoordinates() = default;
        GpsCoordinates(GpsCoordinates&& other) = default;

        explicit GpsCoordinates(Collection<rerun::components::Position3D> _coordinates)
            : coordinates(std::move(_coordinates)) {}

        /// Optional radii for the points, effectively turning them into circles.
        GpsCoordinates with_radii(Collection<rerun::components::Radius> _radii) && {
            radii = std::move(_radii);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional colors for the points.
        GpsCoordinates with_colors(Collection<rerun::components::Color> _colors) && {
            colors = std::move(_colors);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::GpsCoordinates> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(const archetypes::GpsCoordinates& archetype);
    };
} // namespace rerun
