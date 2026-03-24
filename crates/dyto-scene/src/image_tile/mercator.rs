//! Math and utilities for working with Mercator projections (EPSG:3857).
//!
//! See:
//! https://learn.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system
//! https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames

/// The radius of the Earth in meters.
pub const EARTH_RADIUS: f64 = 6378137.0;
/// The minimum latitude in degrees.
pub const MIN_LATITUDE: f64 = -85.05112878;
/// The maximum latitude in degrees.
pub const MAX_LATITUDE: f64 = 85.05112878;
/// The minimum longitude in degrees.
pub const MIN_LONGITUDE: f64 = -180.0;
/// The maximum longitude in degrees.
pub const MAX_LONGITUDE: f64 = 180.0;
/// The size of a tile in pixels.
pub const TILE_SIZE: u32 = 256;
/// The size of a tile as a float in pixels.
pub const TILE_SIZE_FLOAT: f64 = TILE_SIZE as f64;
/// A map of numbers to their quadkey character representation.
pub const QUADKEY_CHAR_MAP: [char; 4] = ['0', '1', '2', '3'];

/// The level of detail of the map.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LevelOfDetail(u8);

impl LevelOfDetail {
    /// The maximum level of detail of the map.
    pub const MAX: Self = Self(23);
    /// The minimum level of detail of the map.
    pub const MIN: Self = Self(1);

    /// Creates a new `LevelOfDetail` from a `u8`.
    /// Clamps the value to the range of valid levels of detail.
    pub fn new(value: u8) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }

    /// Gets the `u8` value of the `LevelOfDetail`.
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Get the map size at the given level of detail in pixels.
    pub fn map_size(&self) -> u32 {
        TILE_SIZE << self.0
    }

    /// Get the ground resolution (meters per pixel) at the given level of
    /// detail for the given latitude.
    pub fn ground_resolution(&self, latitude: f64) -> f64 {
        let latitude = latitude.clamp(MIN_LATITUDE, MAX_LATITUDE);
        latitude.to_radians().cos() * 2.0 * std::f64::consts::PI * EARTH_RADIUS
            / self.map_size() as f64
    }

    /// Get the map scale at the given level of detail for the given latitude
    /// and screen resolution (DPI).
    pub fn map_scale(&self, latitude: f64, screen_dpi: f64) -> f64 {
        self.ground_resolution(latitude) * screen_dpi / 0.0254
    }
}

impl From<LevelOfDetail> for u8 {
    fn from(level_of_detail: LevelOfDetail) -> Self {
        level_of_detail.value()
    }
}

/// A coordinate in WGS 84 (EPSG:4326) format.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WGS84Coordinate {
    /// The latitude in degrees.
    latitude: f64,
    /// The longitude in degrees.
    longitude: f64,
}

impl WGS84Coordinate {
    /// Creates a new `WGS84Coordinate` from a latitude and longitude in
    /// degrees.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude: latitude.clamp(-90.0, 90.0),
            longitude: (longitude + 180.0).rem_euclid(360.0) - 180.0,
        }
    }

    /// Gets the latitude in degrees.
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Gets the longitude in degrees.
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    /// Get the Pixel XY coordinates of the coordinate at a given level of
    /// detail.
    pub fn as_pixel_coordinate(
        &self,
        level_of_detail: LevelOfDetail,
    ) -> PixelCoordinate {
        let latitude = self.latitude.clamp(MIN_LATITUDE, MAX_LATITUDE);
        let longitude = self.longitude.clamp(MIN_LONGITUDE, MAX_LONGITUDE);

        let x = (longitude + 180.0) / 360.0;
        let sin_latitude = latitude.to_radians().sin();
        let y = 0.5
            - ((1.0 + sin_latitude) / (1.0 - sin_latitude)).ln()
                / (4.0 * std::f64::consts::PI);

        let map_size = level_of_detail.map_size() as f64;
        PixelCoordinate::new(
            (x * map_size + 0.5).clamp(0.0, map_size - 1.0),
            (y * map_size + 0.5).clamp(0.0, map_size - 1.0),
            level_of_detail,
        )
    }
}

/// A coordinate in pixel XY
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PixelCoordinate {
    /// The X coordinate in pixels.
    x: f64,
    /// The Y coordinate in pixels.
    y: f64,
    /// The level of detail of the coordinate.
    level_of_detail: LevelOfDetail,
}

impl PixelCoordinate {
    /// Creates a new `PixelCoordinate` from an X and Y coordinate in
    /// pixels and a level of detail.
    pub fn new(x: f64, y: f64, level_of_detail: LevelOfDetail) -> Self {
        Self {
            x,
            y,
            level_of_detail,
        }
    }

    /// Gets the X coordinate in pixels.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Gets the Y coordinate in pixels.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Gets the level of detail of the coordinate.
    pub fn level_of_detail(&self) -> LevelOfDetail {
        self.level_of_detail
    }

    /// Get the latitude and longitude in WGS 84 (EPSG:4326) from the pixel
    /// coordinates.
    pub fn as_wgs84(&self) -> (f64, f64) {
        let map_size = self.level_of_detail.map_size() as f64;
        let x = self.x.clamp(0.0, map_size - 1.0) / map_size - 0.5;
        let y = 0.5 - self.y.clamp(0.0, map_size - 1.0) / map_size;

        (
            90.0 - 360.0 * (-(y * 2.0 * std::f64::consts::PI)).exp().atan()
                / std::f64::consts::PI,
            360.0 * x,
        )
    }

    /// Get the tile coordinates of the tile containing the pixel coordinates.
    pub fn as_tile_coordinate(&self) -> TileCoordinate {
        TileCoordinate::new(
            (self.x / TILE_SIZE_FLOAT).floor() as u32,
            (self.y / TILE_SIZE_FLOAT).floor() as u32,
            self.level_of_detail,
        )
    }

    /// Get the meter coordinates of the pixel coordinates.
    pub fn as_mercator(&self) -> (f64, f64) {
        let map_size = self.level_of_detail.map_size() as f64;
        let x = self.x.clamp(0.0, map_size - 1.0) / map_size - 0.5;
        let y = 0.5 - self.y.clamp(0.0, map_size - 1.0) / map_size;

        (
            x * 2.0 * std::f64::consts::PI * EARTH_RADIUS,
            y * 2.0 * std::f64::consts::PI * EARTH_RADIUS,
        )
    }
}

/// A tile in the Web Mercator (EPSG:3857) tile grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TileCoordinate {
    /// The X coordinate of the tile.
    x: u32,
    /// The Y coordinate of the tile.
    y: u32,
    /// The level of detail of the tile.
    level_of_detail: LevelOfDetail,
}

impl TileCoordinate {
    /// Creates a new `TileCoordinate` from an X and Y coordinate and a level of
    /// detail.
    pub fn new(x: u32, y: u32, level_of_detail: LevelOfDetail) -> Self {
        Self {
            x,
            y,
            level_of_detail,
        }
    }

    /// Gets the X coordinate of the tile.
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Gets the Y coordinate of the tile.
    pub fn y(&self) -> u32 {
        self.y
    }

    /// Gets the level of detail of the tile.
    pub fn level_of_detail(&self) -> LevelOfDetail {
        self.level_of_detail
    }

    /// Get the pixel coordinates of the upper-left pixel of the tile.
    pub fn as_pixel_coordinate(&self) -> PixelCoordinate {
        PixelCoordinate::new(
            self.x as f64 * TILE_SIZE_FLOAT,
            self.y as f64 * TILE_SIZE_FLOAT,
            self.level_of_detail,
        )
    }

    /// Get the world position of the tile in meters from the center of the
    /// tile.
    pub fn as_world_position(&self) -> (f64, f64) {
        let center = PixelCoordinate::new(
            (self.x as f64 + 0.5) * TILE_SIZE_FLOAT,
            (self.y as f64 + 0.5) * TILE_SIZE_FLOAT,
            self.level_of_detail,
        );
        center.as_mercator()
    }
}
