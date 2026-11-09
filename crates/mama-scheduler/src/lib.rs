use mama_api::Budget;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Estimate {
    pub weights_bytes: u64,
    pub kv_cache_bytes: u64,
    pub runtime_bytes: u64,
}

impl Estimate {
    pub fn total(&self) -> Option<u64> {
        if self.weights_bytes == 0 || self.kv_cache_bytes == 0 || self.runtime_bytes == 0 {
            return None;
        }
        self.weights_bytes
            .checked_add(self.kv_cache_bytes)?
            .checked_add(self.runtime_bytes)
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    FitsSnapshot,
    MissingHardware,
    ExceedsConfiguredBudget,
    ExceedsAvailableMemory,
}

#[derive(Serialize)]
pub struct Admission {
    pub outcome: Outcome,
    pub required_ram_bytes: u64,
    pub usable_ram_bytes: Option<u64>,
    pub os_reserve_bytes: Option<u64>,
    pub suggested_threads: Option<usize>,
}

pub fn plan_cpu(
    required_ram_bytes: u64,
    budget: &Budget,
    total_ram: Option<u64>,
    available_ram: Option<u64>,
    logical_cpus: Option<usize>,
) -> Admission {
    const GIB: u64 = 1 << 30;
    // Budget settings are decimal GB. The OS reserve is at least 1 GiB.
    let configured = (budget.max_ram_gb * 1_000_000_000.0) as u64;
    let reserve = total_ram
        .filter(|total| *total > 0)
        .map(|total| (total / 10).max(GIB));
    let free = reserve
        .zip(available_ram)
        .zip(total_ram)
        .map(|((reserve, available), total)| available.min(total).saturating_sub(reserve));
    let threads = logical_cpus
        .filter(|cores| *cores > 0)
        .map(|cores| (cores.saturating_mul(usize::from(budget.cpu_target_percent)) / 100).max(1));
    let outcome = match (free, threads) {
        _ if required_ram_bytes > configured => Outcome::ExceedsConfiguredBudget,
        (Some(free), Some(_)) if required_ram_bytes > free => Outcome::ExceedsAvailableMemory,
        (Some(_), Some(_)) => Outcome::FitsSnapshot,
        _ => Outcome::MissingHardware,
    };
    Admission {
        outcome,
        required_ram_bytes,
        usable_ram_bytes: free.map(|free| configured.min(free)),
        os_reserve_bytes: reserve,
        suggested_threads: threads,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_overflow_and_missing_costs() {
        assert_eq!(
            Estimate {
                weights_bytes: u64::MAX,
                kv_cache_bytes: 1,
                runtime_bytes: 1
            }
            .total(),
            None
        );
        assert_eq!(
            Estimate {
                weights_bytes: 1,
                kv_cache_bytes: 0,
                runtime_bytes: 1
            }
            .total(),
            None
        );
    }

    #[test]
    fn checks_saved_limit_os_reserve_and_missing_probes() {
        let budget = Budget {
            max_ram_gb: 2.0,
            cpu_target_percent: 50,
            ..Budget::default()
        };
        let plan =
            |bytes, available| plan_cpu(bytes, &budget, Some(8_000_000_000), available, Some(8));
        let fits = plan(2_000_000_000, Some(4_000_000_000));
        assert_eq!(fits.outcome, Outcome::FitsSnapshot);
        assert_eq!(fits.suggested_threads, Some(4));
        assert_eq!(
            plan(2_000_000_001, Some(4_000_000_000)).outcome,
            Outcome::ExceedsConfiguredBudget
        );
        assert_eq!(plan(1, Some(1)).outcome, Outcome::ExceedsAvailableMemory);
        assert_eq!(plan(1, None).outcome, Outcome::MissingHardware);
    }
}
