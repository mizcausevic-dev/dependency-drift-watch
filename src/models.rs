use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PackageSnapshot {
    pub id: String,
    pub package_name: String,
    pub ecosystem: String,
    pub current_version: String,
    pub latest_version: String,
    pub release_gap: u16,
    pub lockfile_age_days: u16,
    pub owner: String,
    pub service_tier: String,
    pub adoption_scope: String,
    pub cve_signals: u8,
    pub breaking_changes_pending: bool,
    pub automated_updates: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackageAssessment {
    #[serde(flatten)]
    pub snapshot: PackageSnapshot,
    pub drift_score: u8,
    pub verdict: String,
    pub top_risk: String,
    pub recommendation: String,
    pub exposure_flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardSummary {
    pub package_count: usize,
    pub critical_packages: usize,
    pub watch_packages: usize,
    pub automated_update_coverage_percent: u8,
    pub avg_release_gap: u16,
    pub riskiest_owner_lane: String,
    pub lead_recommendation: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackageCollection {
    pub packages: Vec<PackageAssessment>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OwnerLane {
    pub owner: String,
    pub package_count: usize,
    pub critical_count: usize,
    pub avg_drift_score: f32,
    pub focus_package: String,
    pub ecosystems: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PolicyConfig {
    pub scan_interval_hours: u32,
    pub freshness_window_days: u32,
    pub critical_gap_threshold: u16,
    pub allowed_owners: Vec<String>,
    pub alert_targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditEvent {
    pub timestamp: String,
    pub action: String,
    pub resource: String,
    pub result: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SamplePayload {
    pub summary: DashboardSummary,
    pub highest_risk_package: PackageAssessment,
    pub policy: PolicyConfig,
    pub owner_lanes: Vec<OwnerLane>,
    pub audit_excerpt: Vec<AuditEvent>,
}
