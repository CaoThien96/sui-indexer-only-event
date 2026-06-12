#[cfg(test)]
mod integration {
    use sui_indexer_alt_framework_store_traits::{CommitterWatermark, Connection};

    use crate::postgres::{DbArgs, PostgresStore};

    fn database_url() -> Option<url::Url> {
        std::env::var("DATABASE_URL")
            .ok()
            .and_then(|u| u.parse().ok())
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL and running Postgres"]
    async fn watermark_round_trip() {
        let Some(url) = database_url() else {
            panic!("DATABASE_URL not set");
        };

        let store = PostgresStore::for_write(url, DbArgs::default())
            .await
            .expect("connect to postgres");
        let mut conn = store.connect().await.expect("pool connection");

        let pipeline = "test_watermark_round_trip";
        conn.init_watermark(pipeline, Some(100))
            .await
            .expect("init watermark");

        let wm = CommitterWatermark {
            epoch_hi_inclusive: 1,
            checkpoint_hi_inclusive: 200,
            tx_hi: 0,
            timestamp_ms_hi_inclusive: 1_710_000_000_000,
        };
        let updated = conn
            .set_committer_watermark(pipeline, wm)
            .await
            .expect("set watermark");
        assert!(updated);

        let read = conn
            .committer_watermark(pipeline)
            .await
            .expect("read watermark")
            .expect("watermark should exist");
        assert_eq!(read.checkpoint_hi_inclusive, 200);
    }
}
