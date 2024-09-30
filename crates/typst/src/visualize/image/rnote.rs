use rnote_engine::{
    engine::{
        export::{SelectionExportFormat, SelectionExportPrefs},
        EngineSnapshot,
    },
    Engine,
};
use crate::{diag::StrResult, foundations::Bytes, visualize::SvgImage};

const RNOTE_LOGO: &[u8] = include_bytes!("rnote-logo.svg");

#[tokio::main]
#[comemo::memoize]
pub async fn export_as_svg(data: Bytes) -> StrResult<SvgImage> {
    let mut engine = Engine::default();

    let snapshot = EngineSnapshot::load_from_rnote_bytes(data.to_vec())
        .await
        .map_err(|e| format!("Unable to load Rnote document: {e}"))?;

    let _ = engine.load_snapshot(snapshot);
    let _ = engine.select_all_strokes();

    let svg_bytes = engine
        .export_selection(Some(SelectionExportPrefs {
            export_format: SelectionExportFormat::Svg,
            with_pattern: false,
            with_background: false,
            optimize_printing: false,
            ..Default::default()
        }))
        .await
        .map_err(|_| "Rendering Rnote document got cancelled")?
        .map_err(|e| format!("Unable to render Rnote document: {e}"))?
        .unwrap_or(RNOTE_LOGO.to_vec());

    Ok(SvgImage::new(svg_bytes.into())?)
}
