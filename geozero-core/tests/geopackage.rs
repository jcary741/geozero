#[cfg(feature = "gpkg")]
mod gpkg_sqlx {
    use geozero_core::wkb;
    use geozero_core::wkt::WktWriter;
    use sqlx::sqlite::{SqlitePool, SqliteQueryAs};
    use tokio::runtime::Runtime;

    async fn geometry_columns_query() -> Result<(), sqlx::Error> {
        let pool = SqlitePool::builder()
            .max_size(5)
            .build("sqlite://tests/data/gpkg_test.gpkg")
            .await?;

        let row: (String,String,) = sqlx::query_as(
            "SELECT table_name, column_name, geometry_type_name, srs_id, z, m FROM gpkg_geometry_columns"
            )
            .fetch_one(&pool)
            .await?;

        dbg!(&row);
        assert_eq!(row, ("pt2d".to_string(), "geom".to_string()));

        Ok(())
    }

    #[test]
    fn async_geometry_columns_query() {
        assert_eq!(
            Runtime::new()
                .unwrap()
                .block_on(geometry_columns_query())
                .map_err(|e| e.to_string()),
            Ok(())
        );
    }

    async fn blob_query() -> Result<(), sqlx::Error> {
        let pool = SqlitePool::builder()
            .max_size(5)
            .build("sqlite://tests/data/gpkg_test.gpkg")
            .await?;

        let row: (Vec<u8>,) = sqlx::query_as("SELECT geom FROM pt2d")
            .fetch_one(&pool)
            .await?;

        let mut wkt_data: Vec<u8> = Vec::new();
        let mut writer = WktWriter::new(&mut wkt_data);
        assert!(wkb::process_gpkg_geom(&mut row.0.as_slice(), &mut writer).is_ok());
        assert_eq!(std::str::from_utf8(&wkt_data).unwrap(), "POINT (1.1 1.1)");

        Ok(())
    }

    #[test]
    fn async_blob_query() {
        assert_eq!(
            Runtime::new()
                .unwrap()
                .block_on(blob_query())
                .map_err(|e| e.to_string()),
            Ok(())
        );
    }

    async fn rust_geo_query() -> Result<(), sqlx::Error> {
        use geozero_core::gpkg::geo::Geometry;

        let pool = SqlitePool::builder()
            .max_size(5)
            .build("sqlite://tests/data/gpkg_test.gpkg")
            .await?;

        let row: (Geometry,) = sqlx::query_as("SELECT geom FROM pt2d")
            .fetch_one(&pool)
            .await?;
        let geom = row.0;
        assert_eq!(
            &format!("{:?}", geom.0),
            "Point(Point(Coordinate { x: 1.1, y: 1.1 }))"
        );

        Ok(())
    }

    #[test]
    fn async_rust_geo_query() {
        assert_eq!(
            Runtime::new()
                .unwrap()
                .block_on(rust_geo_query())
                .map_err(|e| e.to_string()),
            Ok(())
        );
    }
}