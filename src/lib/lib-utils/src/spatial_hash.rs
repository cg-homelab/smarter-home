//! Spatial hashing helpers powered by H3.
//!
//! This module provides string-first helpers for common H3 operations used in backend
//! services, while still exposing typed and numeric conversion utilities.
//!
//! Coordinate conventions:
//! - Function parameters use `(lat, lng)` in degrees.
//! - GeoJSON uses `[lng, lat]` position ordering as specified by RFC 7946.
//!
//! # Example
//! ```rust
//! use lib_utils::spatial_hash::lat_lng_to_cell_string;
//!
//! let cell = lat_lng_to_cell_string(48.864716, 2.349014, 10).unwrap();
//! assert!(!cell.is_empty());
//! ```

use geo_types::{Coord, LineString, Polygon};
use geojson::{GeoJson, Value};
use h3o::{
    geom::{ContainmentMode, TilerBuilder},
    CellIndex, LatLng, Resolution,
};
use lib_models::error::SpatialHashError;

/// Containment mode used for polygon coverage (polyfill).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolyfillContainmentMode {
    /// Select cells whose centroid is inside the polygon.
    ContainsCentroid,
    /// Select cells whose boundary is fully inside the polygon.
    ContainsBoundary,
    /// Select cells whose boundary intersects the polygon boundary.
    IntersectsBoundary,
    /// Like `IntersectsBoundary`, but also includes a covering cell when applicable.
    Covers,
}

impl From<PolyfillContainmentMode> for ContainmentMode {
    fn from(value: PolyfillContainmentMode) -> Self {
        match value {
            PolyfillContainmentMode::ContainsCentroid => ContainmentMode::ContainsCentroid,
            PolyfillContainmentMode::ContainsBoundary => ContainmentMode::ContainsBoundary,
            PolyfillContainmentMode::IntersectsBoundary => ContainmentMode::IntersectsBoundary,
            PolyfillContainmentMode::Covers => ContainmentMode::Covers,
        }
    }
}

/// Parse an H3 resolution from a `u8`.
pub fn parse_resolution(resolution: u8) -> Result<Resolution, SpatialHashError> {
    Resolution::try_from(resolution).map_err(|_| SpatialHashError::InvalidResolution { resolution })
}

/// Parse a cell index from its canonical hexadecimal string form.
pub fn parse_cell(cell: &str) -> Result<CellIndex, SpatialHashError> {
    cell.parse::<CellIndex>()
        .map_err(|_| SpatialHashError::InvalidCell {
            cell: cell.to_owned(),
        })
}

/// Parse a cell index from its raw `u64` representation.
pub fn parse_cell_u64(cell: u64) -> Result<CellIndex, SpatialHashError> {
    CellIndex::try_from(cell).map_err(|_| SpatialHashError::InvalidCellU64 { index: cell })
}

/// Convert a `(lat, lng)` coordinate to a typed H3 cell index.
pub fn lat_lng_to_cell(lat: f64, lng: f64, resolution: u8) -> Result<CellIndex, SpatialHashError> {
    let resolution = parse_resolution(resolution)?;
    let lat_lng = parse_lat_lng(lat, lng)?;
    Ok(lat_lng.to_cell(resolution))
}

/// Convert a `(lat, lng)` coordinate to its canonical H3 hexadecimal cell string.
pub fn lat_lng_to_cell_string(
    lat: f64,
    lng: f64,
    resolution: u8,
) -> Result<String, SpatialHashError> {
    Ok(lat_lng_to_cell(lat, lng, resolution)?.to_string())
}

/// Convert a `(lat, lng)` coordinate to its raw `u64` H3 index.
pub fn lat_lng_to_cell_u64(lat: f64, lng: f64, resolution: u8) -> Result<u64, SpatialHashError> {
    Ok(u64::from(lat_lng_to_cell(lat, lng, resolution)?))
}

/// Return the parent cell at `parent_resolution` for a cell string.
pub fn parent_cell(cell: &str, parent_resolution: u8) -> Result<String, SpatialHashError> {
    let cell = parse_cell(cell)?;
    let resolution = parse_resolution(parent_resolution)?;
    if resolution > cell.resolution() {
        return Err(SpatialHashError::InvalidParentResolution {
            parent_resolution,
            cell_resolution: u8::from(cell.resolution()),
        });
    }

    cell.parent(resolution)
        .map(|index| index.to_string())
        .ok_or(SpatialHashError::InvalidParentResolution {
            parent_resolution,
            cell_resolution: u8::from(cell.resolution()),
        })
}

/// Return all descendant cells for a parent at `child_resolution`.
pub fn child_cells(cell: &str, child_resolution: u8) -> Result<Vec<String>, SpatialHashError> {
    let cell = parse_cell(cell)?;
    let resolution = parse_resolution(child_resolution)?;
    if resolution < cell.resolution() {
        return Err(SpatialHashError::InvalidChildResolution {
            child_resolution,
            cell_resolution: u8::from(cell.resolution()),
        });
    }

    Ok(cell
        .children(resolution)
        .map(|index| index.to_string())
        .collect())
}

/// Alias for `child_cells` with a name that emphasizes containment by a coarser parent.
pub fn cells_within_parent(
    parent_cell: &str,
    child_resolution: u8,
) -> Result<Vec<String>, SpatialHashError> {
    child_cells(parent_cell, child_resolution)
}

/// Return all cells within `k` grid distance of `cell`.
///
/// `k=0` returns only the source cell.
pub fn k_ring(cell: &str, k: u32) -> Result<Vec<String>, SpatialHashError> {
    let cell = parse_cell(cell)?;
    Ok(cell
        .grid_disk::<Vec<_>>(k)
        .into_iter()
        .map(|index| index.to_string())
        .collect())
}

/// Compute grid distance between two cells.
pub fn grid_distance(from_cell: &str, to_cell: &str) -> Result<i32, SpatialHashError> {
    let from_cell = parse_cell(from_cell)?;
    let to_cell = parse_cell(to_cell)?;
    from_cell
        .grid_distance(to_cell)
        .map_err(|error| SpatialHashError::GridDistance {
            details: error.to_string(),
        })
}

/// Compute great-circle distance in kilometers between two coordinates.
pub fn great_circle_distance_km(
    from_lat: f64,
    from_lng: f64,
    to_lat: f64,
    to_lng: f64,
) -> Result<f64, SpatialHashError> {
    let from = parse_lat_lng(from_lat, from_lng)?;
    let to = parse_lat_lng(to_lat, to_lng)?;
    Ok(from.distance_km(to))
}

/// Return average H3 edge length (in kilometers) for a resolution.
pub fn average_edge_length_km(resolution: u8) -> Result<f64, SpatialHashError> {
    Ok(parse_resolution(resolution)?.edge_length_km())
}

/// Return area in km² for a specific cell.
pub fn cell_area_km2(cell: &str) -> Result<f64, SpatialHashError> {
    Ok(parse_cell(cell)?.area_km2())
}

/// Polyfill a GeoJSON Polygon or MultiPolygon payload into H3 cells.
///
/// The resulting list is sorted and deduplicated for deterministic behavior.
pub fn geojson_to_cells(
    geojson: &str,
    resolution: u8,
    mode: PolyfillContainmentMode,
) -> Result<Vec<String>, SpatialHashError> {
    let resolution = parse_resolution(resolution)?;
    let parsed: GeoJson =
        geojson
            .parse::<GeoJson>()
            .map_err(|error| SpatialHashError::InvalidGeoJson {
                details: error.to_string(),
            })?;

    let polygons = extract_polygons(parsed)?;
    let mut tiler = TilerBuilder::new(resolution)
        .containment_mode(mode.into())
        .build();

    tiler
        .add_batch(polygons)
        .map_err(|error| SpatialHashError::InvalidGeometry {
            details: error.to_string(),
        })?;

    let mut cells = tiler
        .into_coverage()
        .map(|index| index.to_string())
        .collect::<Vec<_>>();

    cells.sort_unstable();
    cells.dedup();
    Ok(cells)
}

/// Polyfill a GeoJSON polygon payload using `ContainsCentroid` mode.
pub fn geojson_to_cells_default(
    geojson: &str,
    resolution: u8,
) -> Result<Vec<String>, SpatialHashError> {
    geojson_to_cells(
        geojson,
        resolution,
        PolyfillContainmentMode::ContainsCentroid,
    )
}

/// Convert many coordinates to H3 cells in a single pass.
///
/// This function preserves input ordering and returns per-item conversion results.
pub fn lat_lng_to_cell_strings_batch(
    coordinates: &[(f64, f64)],
    resolution: u8,
) -> Vec<Result<String, SpatialHashError>> {
    coordinates
        .iter()
        .map(|(lat, lng)| lat_lng_to_cell_string(*lat, *lng, resolution))
        .collect()
}

/// Convert many coordinates to H3 cells concurrently using spawned async tasks.
///
/// This function preserves input ordering. Any task join failure returns an error.
pub async fn lat_lng_to_cell_strings_batch_async(
    coordinates: &[(f64, f64)],
    resolution: u8,
) -> Result<Vec<Result<String, SpatialHashError>>, SpatialHashError> {
    let mut join_set = tokio::task::JoinSet::new();

    for (index, (lat, lng)) in coordinates.iter().copied().enumerate() {
        join_set.spawn(async move { (index, lat_lng_to_cell_string(lat, lng, resolution)) });
    }

    let mut ordered: Vec<Option<Result<String, SpatialHashError>>> =
        (0..coordinates.len()).map(|_| None).collect();

    while let Some(join_result) = join_set.join_next().await {
        match join_result {
            Ok((index, result)) => {
                ordered[index] = Some(result);
            }
            Err(error) => {
                return Err(SpatialHashError::AsyncBatchJoin {
                    details: error.to_string(),
                });
            }
        }
    }

    Ok(ordered
        .into_iter()
        .map(|entry| {
            entry.unwrap_or_else(|| {
                Err(SpatialHashError::AsyncBatchJoin {
                    details: "task did not produce output".to_owned(),
                })
            })
        })
        .collect())
}

fn parse_lat_lng(lat: f64, lng: f64) -> Result<LatLng, SpatialHashError> {
    LatLng::new(lat, lng).map_err(|_| SpatialHashError::InvalidLatLng { lat, lng })
}

fn extract_polygons(geojson: GeoJson) -> Result<Vec<Polygon<f64>>, SpatialHashError> {
    let mut polygons = Vec::new();

    match geojson {
        GeoJson::Geometry(geometry) => {
            polygons.extend(polygons_from_value(&geometry.value)?);
        }
        GeoJson::Feature(feature) => {
            let geometry = feature
                .geometry
                .ok_or(SpatialHashError::MissingPolygonGeometry)?;
            polygons.extend(polygons_from_value(&geometry.value)?);
        }
        GeoJson::FeatureCollection(collection) => {
            for feature in collection.features {
                if let Some(geometry) = feature.geometry {
                    polygons.extend(polygons_from_value(&geometry.value)?);
                }
            }
        }
    }

    if polygons.is_empty() {
        return Err(SpatialHashError::MissingPolygonGeometry);
    }

    Ok(polygons)
}

fn polygons_from_value(value: &Value) -> Result<Vec<Polygon<f64>>, SpatialHashError> {
    match value {
        Value::Polygon(rings) => Ok(vec![polygon_from_rings(rings)?]),
        Value::MultiPolygon(polygons) => polygons
            .iter()
            .map(|rings| polygon_from_rings(rings))
            .collect::<Result<Vec<_>, _>>(),
        Value::GeometryCollection(geometries) => {
            let mut polygons = Vec::new();
            for geometry in geometries {
                polygons.extend(polygons_from_value(&geometry.value)?);
            }
            Ok(polygons)
        }
        _ => Err(SpatialHashError::UnsupportedGeoJsonGeometry {
            geometry_type: geojson_value_type_name(value).to_owned(),
        }),
    }
}

fn polygon_from_rings(rings: &[Vec<Vec<f64>>]) -> Result<Polygon<f64>, SpatialHashError> {
    let (outer, holes) = rings
        .split_first()
        .ok_or(SpatialHashError::InvalidGeoJson {
            details: "polygon has no linear rings".to_owned(),
        })?;

    let outer = line_string_from_ring(outer)?;
    let holes = holes
        .iter()
        .map(|ring| line_string_from_ring(ring))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Polygon::new(outer, holes))
}

fn line_string_from_ring(ring: &[Vec<f64>]) -> Result<LineString<f64>, SpatialHashError> {
    let mut coords = ring
        .iter()
        .enumerate()
        .map(|(index, position)| coord_from_position(position, index))
        .collect::<Result<Vec<_>, _>>()?;

    if coords.len() < 3 {
        return Err(SpatialHashError::InvalidGeoJson {
            details: "polygon ring must contain at least 3 positions".to_owned(),
        });
    }

    if let (Some(first), Some(last)) = (coords.first().copied(), coords.last().copied()) {
        if first != last {
            coords.push(first);
        }
    }

    Ok(LineString::from(coords))
}

fn coord_from_position(position: &[f64], index: usize) -> Result<Coord<f64>, SpatialHashError> {
    if position.len() < 2 {
        return Err(SpatialHashError::InvalidGeoJsonCoordinate {
            index,
            details: "position must contain at least [lng, lat]".to_owned(),
        });
    }

    let lng = position[0];
    let lat = position[1];

    if !lng.is_finite() || !lat.is_finite() {
        return Err(SpatialHashError::InvalidGeoJsonCoordinate {
            index,
            details: "coordinates must be finite numbers".to_owned(),
        });
    }

    Ok(Coord { x: lng, y: lat })
}

fn geojson_value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Point(_) => "Point",
        Value::MultiPoint(_) => "MultiPoint",
        Value::LineString(_) => "LineString",
        Value::MultiLineString(_) => "MultiLineString",
        Value::Polygon(_) => "Polygon",
        Value::MultiPolygon(_) => "MultiPolygon",
        Value::GeometryCollection(_) => "GeometryCollection",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PARIS_LAT: f64 = 48.864716;
    const PARIS_LNG: f64 = 2.349014;

    #[test]
    fn lat_lng_to_cell_is_deterministic() {
        let first = lat_lng_to_cell_string(PARIS_LAT, PARIS_LNG, 10).unwrap();
        let second = lat_lng_to_cell_string(PARIS_LAT, PARIS_LNG, 10).unwrap();

        assert_eq!(first, second);
        assert!(!first.is_empty());
    }

    #[test]
    fn parent_and_children_roundtrip_contains_original_cell() {
        let cell = lat_lng_to_cell_string(PARIS_LAT, PARIS_LNG, 10).unwrap();
        let parent = parent_cell(&cell, 9).unwrap();
        let children = child_cells(&parent, 10).unwrap();

        assert!(children.contains(&cell));
    }

    #[test]
    fn k_ring_with_zero_only_returns_origin() {
        let cell = lat_lng_to_cell_string(PARIS_LAT, PARIS_LNG, 9).unwrap();
        let ring = k_ring(&cell, 0).unwrap();

        assert_eq!(ring.len(), 1);
        assert_eq!(ring[0], cell);
    }

    #[test]
    fn grid_distance_to_self_is_zero() {
        let cell = lat_lng_to_cell_string(PARIS_LAT, PARIS_LNG, 9).unwrap();
        let distance = grid_distance(&cell, &cell).unwrap();

        assert_eq!(distance, 0);
    }

    #[test]
    fn great_circle_distance_is_positive_for_distinct_points() {
        let distance =
            great_circle_distance_km(48.864716, 2.349014, 31.224361, 121.469170).unwrap();
        assert!(distance > 0.0);
    }

    #[test]
    fn geojson_polyfill_returns_cells_for_polygon() {
        let polygon = r#"{
            "type": "Polygon",
            "coordinates": [[[2.33, 48.86], [2.35, 48.86], [2.35, 48.87], [2.33, 48.87], [2.33, 48.86]]]
        }"#;

        let cells = geojson_to_cells_default(polygon, 10).unwrap();
        assert!(!cells.is_empty());
    }

    #[test]
    fn geojson_polyfill_rejects_unsupported_geometry() {
        let point = r#"{
            "type": "Point",
            "coordinates": [2.349014, 48.864716]
        }"#;

        let error = geojson_to_cells_default(point, 10).unwrap_err();
        assert!(matches!(
            error,
            SpatialHashError::UnsupportedGeoJsonGeometry { .. }
        ));
    }

    #[test]
    fn sync_batch_keeps_input_order() {
        let coordinates = vec![(48.864716, 2.349014), (31.224361, 121.469170)];
        let batch = lat_lng_to_cell_strings_batch(&coordinates, 8);

        assert_eq!(batch.len(), 2);
        assert!(batch[0].as_ref().is_ok());
        assert!(batch[1].as_ref().is_ok());
        assert_ne!(batch[0].as_ref().unwrap(), batch[1].as_ref().unwrap());
    }

    #[tokio::test]
    async fn async_batch_matches_sync_batch_results() {
        let coordinates = vec![(48.864716, 2.349014), (31.224361, 121.469170)];

        let sync_results = lat_lng_to_cell_strings_batch(&coordinates, 8);
        let async_results = lat_lng_to_cell_strings_batch_async(&coordinates, 8)
            .await
            .unwrap();

        assert_eq!(sync_results.len(), async_results.len());

        for (sync, async_result) in sync_results.iter().zip(async_results.iter()) {
            assert_eq!(sync.as_ref().ok(), async_result.as_ref().ok());
        }
    }
}
