// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/gps_coordinates.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Set of GpsCoordinates
///
/// ## Example
///
/// ### Disconnected space
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_disconnected_space").spawn()?;
///
///     // These two points can be projected into the same space..
///     rec.log(
///         "world/room1/point",
///         &rerun::Points3D::new([(0.0, 0.0, 0.0)]),
///     )?;
///     rec.log(
///         "world/room2/point",
///         &rerun::Points3D::new([(1.0, 1.0, 1.0)]),
///     )?;
///
///     // ..but this one lives in a completely separate space!
///     rec.log("world/wormhole", &rerun::DisconnectedSpace::new(true))?;
///     rec.log(
///         "world/wormhole/point",
///         &rerun::Points3D::new([(2.0, 2.0, 2.0)]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/1200w.png">
///   <img src="https://static.rerun.io/disconnected_space/b8f95b0e32359de625a765247c84935146c1fba9/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct GpsCoordinates {
    /// All the GpsCoordinates
    pub coordinates: Vec<crate::components::Position3D>,

    /// Optional radii for the points, effectively turning them into circles.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the points.
    pub colors: Option<Vec<crate::components::Color>>,
}

impl ::re_types_core::SizeBytes for GpsCoordinates {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.coordinates.heap_size_bytes()
            + self.radii.heap_size_bytes()
            + self.colors.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::Position3D>>::is_pod()
            && <Option<Vec<crate::components::Radius>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Position3D".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Radius".into(),
            "rerun.components.Color".into(),
            "rerun.components.GpsCoordinatesIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Position3D".into(),
            "rerun.components.Radius".into(),
            "rerun.components.Color".into(),
            "rerun.components.GpsCoordinatesIndicator".into(),
        ]
    });

impl GpsCoordinates {
    /// The total number of components in the archetype: 1 required, 3 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 4usize;
}

/// Indicator component for the [`GpsCoordinates`] [`::re_types_core::Archetype`]
pub type GpsCoordinatesIndicator = ::re_types_core::GenericIndicatorComponent<GpsCoordinates>;

impl ::re_types_core::Archetype for GpsCoordinates {
    type Indicator = GpsCoordinatesIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.GpsCoordinates".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Gps coordinates"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: GpsCoordinatesIndicator = GpsCoordinatesIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let coordinates = {
            let array = arrays_by_name
                .get("rerun.components.Position3D")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.GpsCoordinates#coordinates")?;
            <crate::components::Position3D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.GpsCoordinates#coordinates")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.GpsCoordinates#coordinates")?
        };
        let radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.GpsCoordinates#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GpsCoordinates#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.GpsCoordinates#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GpsCoordinates#colors")?
            })
        } else {
            None
        };
        Ok(Self {
            coordinates,
            radii,
            colors,
        })
    }
}

impl ::re_types_core::AsComponents for GpsCoordinates {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.coordinates as &dyn ComponentBatch).into()),
            self.radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl GpsCoordinates {
    /// Create a new `GpsCoordinates`.
    #[inline]
    pub fn new(
        coordinates: impl IntoIterator<Item = impl Into<crate::components::Position3D>>,
    ) -> Self {
        Self {
            coordinates: coordinates.into_iter().map(Into::into).collect(),
            radii: None,
            colors: None,
        }
    }

    /// Optional radii for the points, effectively turning them into circles.
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the points.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }
}
