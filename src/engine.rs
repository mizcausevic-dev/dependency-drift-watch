use crate::data;
use crate::models::{
    AuditEvent, DashboardSummary, OwnerLane, PackageAssessment, PackageCollection, PackageSnapshot,
    PolicyConfig, SamplePayload,
};

pub fn dashboard_summary() -> DashboardSummary {
    let packages = packages();
    let package_count = packages.len();
    let critical_packages = packages.iter().filter(|pkg| pkg.verdict == "critical").count();
    let watch_packages = packages.iter().filter(|pkg| pkg.verdict == "watch").count();
    let automated_update_coverage_percent = ((packages
        .iter()
        .filter(|pkg| pkg.snapshot.automated_updates)
        .count()
        * 100)
        / package_count) as u8;
    let avg_release_gap =
        (packages.iter().map(|pkg| pkg.snapshot.release_gap as u32).sum::<u32>() / package_count as u32)
            as u16;
    let riskiest_owner_lane = owner_lanes()
        .first()
        .map(|lane| lane.owner.clone())
        .unwrap_or_else(|| "none".into());

    DashboardSummary {
        package_count,
        critical_packages,
        watch_packages,
        automated_update_coverage_percent,
        avg_release_gap,
        riskiest_owner_lane,
        lead_recommendation: "Clear tier-0 Java drift and stacked frontend major-version gaps before the next release window turns package lag into production risk.".into(),
    }
}

pub fn packages() -> Vec<PackageAssessment> {
    let mut rows: Vec<PackageAssessment> = data::package_snapshots().into_iter().map(assess).collect();
    rows.sort_by(|a, b| b.drift_score.cmp(&a.drift_score));
    rows
}

pub fn package(id: &str) -> Option<PackageAssessment> {
    data::package_snapshots()
        .into_iter()
        .map(assess)
        .find(|pkg| pkg.snapshot.id == id)
}

pub fn package_collection() -> PackageCollection {
    PackageCollection { packages: packages() }
}

pub fn review_queue() -> Vec<PackageAssessment> {
    packages()
        .into_iter()
        .filter(|pkg| pkg.verdict != "healthy")
        .collect()
}

pub fn owner_lanes() -> Vec<OwnerLane> {
    let mut lanes: Vec<String> = data::package_snapshots()
        .iter()
        .map(|snapshot| snapshot.owner.clone())
        .collect::<Vec<_>>();
    lanes.sort();
    lanes.dedup();

    let mut rows: Vec<OwnerLane> = lanes
        .into_iter()
        .map(|owner| {
            let owned: Vec<PackageAssessment> = packages()
                .into_iter()
                .filter(|pkg| pkg.snapshot.owner == owner)
                .collect();

            let package_count = owned.len();
            let critical_count = owned.iter().filter(|pkg| pkg.verdict == "critical").count();
            let avg_drift_score = owned.iter().map(|pkg| pkg.drift_score as f32).sum::<f32>()
                / package_count as f32;
            let focus_package = owned
                .first()
                .map(|pkg| pkg.snapshot.package_name.clone())
                .unwrap_or_else(|| "none".into());
            let mut ecosystems: Vec<String> =
                owned.iter().map(|pkg| pkg.snapshot.ecosystem.clone()).collect();
            ecosystems.sort();
            ecosystems.dedup();

            OwnerLane {
                owner,
                package_count,
                critical_count,
                avg_drift_score: (avg_drift_score * 10.0).round() / 10.0,
                focus_package,
                ecosystems,
            }
        })
        .collect();

    rows.sort_by(|a, b| b.avg_drift_score.total_cmp(&a.avg_drift_score));
    rows
}

pub fn policy_config() -> PolicyConfig {
    data::policy_config()
}

pub fn audit_log() -> Vec<AuditEvent> {
    data::audit_log()
}

pub fn sample_payload() -> SamplePayload {
    SamplePayload {
        summary: dashboard_summary(),
        highest_risk_package: packages()
            .first()
            .cloned()
            .expect("sample package set should not be empty"),
        policy: policy_config(),
        owner_lanes: owner_lanes().into_iter().take(3).collect(),
        audit_excerpt: audit_log().into_iter().take(3).collect(),
    }
}

pub fn prometheus_metrics() -> String {
    let mut lines = vec![
        "# HELP dependency_drift_score Drift-oriented risk score for each package.".to_string(),
        "# TYPE dependency_drift_score gauge".to_string(),
        "# HELP dependency_release_gap_count Number of releases behind the latest known version.".to_string(),
        "# TYPE dependency_release_gap_count gauge".to_string(),
        "# HELP dependency_lockfile_age_days Lockfile age in days.".to_string(),
        "# TYPE dependency_lockfile_age_days gauge".to_string(),
        "# HELP dependency_cve_signal_count Count of active CVE-related pressure signals.".to_string(),
        "# TYPE dependency_cve_signal_count gauge".to_string(),
        "# HELP dependency_automated_updates_enabled Whether automated updates are enabled for the package.".to_string(),
        "# TYPE dependency_automated_updates_enabled gauge".to_string(),
    ];

    for pkg in packages() {
        let labels = format!(
            "package=\"{}\",ecosystem=\"{}\",owner=\"{}\",tier=\"{}\",verdict=\"{}\"",
            pkg.snapshot.package_name,
            pkg.snapshot.ecosystem,
            pkg.snapshot.owner,
            pkg.snapshot.service_tier,
            pkg.verdict
        );
        lines.push(format!("dependency_drift_score{{{labels}}} {}", pkg.drift_score));
        lines.push(format!(
            "dependency_release_gap_count{{{labels}}} {}",
            pkg.snapshot.release_gap
        ));
        lines.push(format!(
            "dependency_lockfile_age_days{{{labels}}} {}",
            pkg.snapshot.lockfile_age_days
        ));
        lines.push(format!(
            "dependency_cve_signal_count{{{labels}}} {}",
            pkg.snapshot.cve_signals
        ));
        lines.push(format!(
            "dependency_automated_updates_enabled{{{labels}}} {}",
            if pkg.snapshot.automated_updates { 1 } else { 0 }
        ));
    }

    lines.join("\n")
}

fn assess(snapshot: PackageSnapshot) -> PackageAssessment {
    let mut drift = 12u8;
    let mut flags = Vec::new();

    if snapshot.release_gap >= 8 {
        drift += 24;
        flags.push("large release gap".into());
    } else if snapshot.release_gap >= 5 {
        drift += 15;
        flags.push("release lag".into());
    }

    if snapshot.lockfile_age_days >= 90 {
        drift += 18;
        flags.push("stale lockfile".into());
    } else if snapshot.lockfile_age_days >= 45 {
        drift += 10;
        flags.push("aging lockfile".into());
    }

    if snapshot.cve_signals >= 3 {
        drift += 24;
        flags.push("security pressure".into());
    } else if snapshot.cve_signals >= 1 {
        drift += 10;
        flags.push("cve attention".into());
    }

    if snapshot.breaking_changes_pending {
        drift += 14;
        flags.push("major-version review".into());
    }

    if !snapshot.automated_updates {
        drift += 10;
        flags.push("no auto-updates".into());
    }

    if snapshot.service_tier == "tier-0" {
        drift += 14;
        flags.push("tier-0 blast radius".into());
    } else if snapshot.service_tier == "tier-1" {
        drift += 8;
    }

    let drift_score = drift.min(100);
    let verdict = if drift_score >= 78 {
        "critical"
    } else if drift_score >= 48 {
        "watch"
    } else {
        "healthy"
    };

    let top_risk = if snapshot.cve_signals >= 3 {
        "This package is carrying both release lag and active security pressure, so it should not wait for a routine dependency sweep."
    } else if snapshot.breaking_changes_pending && snapshot.release_gap >= 8 {
        "The package is multiple releases behind and the next move includes a major-version jump that deserves explicit owner review."
    } else if snapshot.lockfile_age_days >= 90 {
        "Lockfile age is high enough that drift may already be hiding inside transitive dependencies."
    } else {
        "The package is aging into risk faster than the owner lane is refreshing it."
    };

    let recommendation = match verdict {
        "critical" => {
            "Open a dedicated remediation lane, refresh the lockfile, and review breaking-change notes before the next release window."
        }
        "watch" => {
            "Schedule an owner review soon, validate release notes, and move the package back into an automated freshness cadence."
        }
        _ => "Keep the package in the standard update cadence and preserve current review thresholds.",
    };

    PackageAssessment {
        snapshot,
        drift_score,
        verdict: verdict.into(),
        top_risk: top_risk.into(),
        recommendation: recommendation.into(),
        exposure_flags: flags,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log4j_goes_critical() {
        let pkg = package("pkg-log4j").expect("package should exist");
        assert_eq!(pkg.verdict, "critical");
        assert!(pkg.snapshot.cve_signals >= 3);
    }

    #[test]
    fn metrics_include_expected_series() {
        let payload = prometheus_metrics();
        assert!(payload.contains("dependency_drift_score"));
        assert!(payload.contains("package=\"fastapi\""));
    }
}
