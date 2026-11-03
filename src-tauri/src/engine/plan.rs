//! Dry-run plans: what applying would change, shown to the user before anything happens.
//!
//! A plan is bound to the values observed when it was built. Applying re-reads every
//! target and refuses if anything moved, so the user only ever confirms the real diff.
//! Plans expire and are single-use.

use std::collections::HashMap;
use std::sync::Mutex;

use serde::Serialize;

use crate::engine::journal::now_ms;
use crate::engine::setting::{RegTarget, RegValue, describe};
use crate::engine::{Compatibility, RestartRequirement, RiskLevel};
use crate::error::{AppError, AppResult};

pub const PLAN_TTL_MS: i64 = 5 * 60 * 1000;
const MAX_PENDING_PLANS: usize = 16;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannedChange {
    pub tweak_id: String,
    pub target: RegTarget,
    /// Display form, e.g. `HKCU\System\GameConfigStore\GameDVR_Enabled`.
    pub target_label: String,
    pub from: Option<RegValue>,
    pub to: Option<RegValue>,
    pub from_label: String,
    pub to_label: String,
}

impl PlannedChange {
    pub fn new(
        tweak_id: &str,
        target: RegTarget,
        from: Option<RegValue>,
        to: Option<RegValue>,
    ) -> Self {
        PlannedChange {
            tweak_id: tweak_id.to_string(),
            target_label: target.to_string(),
            from_label: describe(&from),
            to_label: describe(&to),
            target,
            from,
            to,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkippedTweak {
    pub tweak_id: String,
    pub reason: Compatibility,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    /// Tweaks with at least one change, in apply order.
    pub tweak_ids: Vec<String>,
    pub changes: Vec<PlannedChange>,
    /// Requested tweaks whose settings already match.
    pub already_applied: Vec<String>,
    pub skipped: Vec<SkippedTweak>,
    pub highest_risk: Option<RiskLevel>,
    pub restart: RestartRequirement,
    /// Applying will show one UAC prompt for the elevated helper.
    pub needs_elevation: bool,
}

/// Pending plans, held in memory only.
#[derive(Default)]
pub struct PlanStore {
    plans: Mutex<HashMap<String, Plan>>,
}

impl PlanStore {
    pub fn insert(&self, plan: Plan) {
        let mut plans = self.plans.lock().unwrap();
        let now = now_ms();
        plans.retain(|_, p| p.expires_at_ms > now);
        if plans.len() >= MAX_PENDING_PLANS {
            // Oldest first: a flood of plan requests cannot grow memory without bound.
            if let Some(oldest) = plans
                .values()
                .min_by_key(|p| p.created_at_ms)
                .map(|p| p.id.clone())
            {
                plans.remove(&oldest);
            }
        }
        plans.insert(plan.id.clone(), plan);
    }

    /// Removes and returns the plan. Single use: a second call fails.
    pub fn take(&self, id: &str) -> AppResult<Plan> {
        let plan = self
            .plans
            .lock()
            .unwrap()
            .remove(id)
            .ok_or_else(|| AppError::InvalidInput("unknown or already used plan".into()))?;
        if plan.expires_at_ms <= now_ms() {
            return Err(AppError::Conflict(
                "the plan expired; review the changes again".into(),
            ));
        }
        Ok(plan)
    }
}

/// 128 random bits, hex. Plan IDs are unguessable so a stale UI cannot apply a plan it did not get.
pub fn new_plan_id() -> AppResult<String> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|e| AppError::Internal(format!("random source: {e}")))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

pub fn is_plan_id(id: &str) -> bool {
    id.len() == 32
        && id
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(id: &str, expires_at_ms: i64) -> Plan {
        Plan {
            id: id.into(),
            created_at_ms: now_ms(),
            expires_at_ms,
            tweak_ids: vec![],
            changes: vec![],
            already_applied: vec![],
            skipped: vec![],
            highest_risk: None,
            restart: RestartRequirement::None,
            needs_elevation: false,
        }
    }

    #[test]
    fn plans_are_single_use_and_expire() {
        let store = PlanStore::default();
        store.insert(plan("a", now_ms() + PLAN_TTL_MS));
        assert!(store.take("a").is_ok());
        assert!(store.take("a").is_err());

        store.insert(plan("b", now_ms() + 50));
        // Force expiry without sleeping.
        store
            .plans
            .lock()
            .unwrap()
            .get_mut("b")
            .unwrap()
            .expires_at_ms = now_ms() - 1;
        assert!(matches!(store.take("b"), Err(AppError::Conflict(_))));
    }

    #[test]
    fn pending_plans_are_bounded() {
        let store = PlanStore::default();
        for i in 0..(MAX_PENDING_PLANS + 5) {
            store.insert(plan(&format!("p{i}"), now_ms() + PLAN_TTL_MS));
        }
        assert_eq!(store.plans.lock().unwrap().len(), MAX_PENDING_PLANS);
    }

    #[test]
    fn plan_ids_are_random_lower_hex() {
        let a = new_plan_id().unwrap();
        assert!(is_plan_id(&a));
        assert_ne!(a, new_plan_id().unwrap());
        assert!(!is_plan_id("ABC"));
        assert!(!is_plan_id(&"g".repeat(32)));
    }
}
