use prometheus::{HistogramVec, IntCounterVec, Registry};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiMetrics {
    pub request_duration: HistogramVec,
    pub cache_hits: IntCounterVec,
    pub errors: IntCounterVec,
}

impl ApiMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Arc<Self>> {
        let request_duration = HistogramVec::new(
            prometheus::HistogramOpts::new(
                "api_request_duration_seconds",
                "API request duration in seconds",
            )
            .buckets(vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5]),
            &["route", "method"],
        )?;
        let cache_hits = IntCounterVec::new(
            prometheus::Opts::new("api_cache_hits_total", "Redis cache hits"),
            &["cache"],
        )?;
        let errors = IntCounterVec::new(
            prometheus::Opts::new("api_errors_total", "API errors by route and status"),
            &["route", "status"],
        )?;
        registry.register(Box::new(request_duration.clone()))?;
        registry.register(Box::new(cache_hits.clone()))?;
        registry.register(Box::new(errors.clone()))?;
        Ok(Arc::new(Self {
            request_duration,
            cache_hits,
            errors,
        }))
    }
}
