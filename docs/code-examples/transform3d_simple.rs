//! Log different transforms between three arrows.
use rerun::{
    components::{Arrow3D, Transform3D},
    datatypes::{Mat3x3, Vec3D},
    transform::{
        Angle, Rotation3D, RotationAxisAngle, Scale3D, TranslationAndMat3x3,
        TranslationRotationScale3D,
    },
    MsgSender, RecordingStreamBuilder,
};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rec_stream, storage) = RecordingStreamBuilder::new("transform").memory()?;

    let arrow = Arrow3D::new(Vec3D::ZERO, (0.0, 1.0, 0.0));

    MsgSender::new("base")
        .with_component(&[arrow])?
        .send(&rec_stream)?;

    MsgSender::new("base/translated")
        .with_component(&[Transform3D::new(TranslationAndMat3x3::new(
            Vec3D::from([1.0, 0.0, 0.0]),
            Mat3x3::IDENTITY,
        ))])?
        .send(&rec_stream)?;

    MsgSender::new("base/translated")
        .with_component(&[arrow])?
        .send(&rec_stream)?;

    MsgSender::new("base/rotated_scaled")
        .with_component(&[Transform3D::new(TranslationRotationScale3D {
            translation: None,
            rotation: Some(Rotation3D::from(RotationAxisAngle::new(
                Vec3D::new(0.0, 0.0, 1.0),
                Angle::Radians(PI / 4.),
            ))),
            scale: Some(Scale3D::from(2.0)),
            ..Default::default()
        })])?
        .send(&rec_stream)?;

    MsgSender::new("base/rotated_scaled")
        .with_component(&[arrow])?
        .send(&rec_stream)?;

    rerun::native_viewer::show(storage.take())?;
    Ok(())
}
