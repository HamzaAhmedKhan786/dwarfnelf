use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Budget {
    pub max_ram_gb: f64,
    #[serde(default)]
    pub max_vram_gb: f64,
    pub max_ssd_gb: f64,
    #[serde(default = "default_cpu_target")]
    pub cpu_target_percent: u8,
}

fn default_cpu_target() -> u8 {
    100
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            max_ram_gb: 1.0,
            max_vram_gb: 0.0,
            max_ssd_gb: 1.0,
            cpu_target_percent: default_cpu_target(),
        }
    }
}

impl Budget {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.max_ram_gb.is_finite() || self.max_ram_gb <= 0.0 {
            return Err("max_ram_gb must be finite and greater than zero");
        }
        if !self.max_vram_gb.is_finite() || self.max_vram_gb < 0.0 {
            return Err("max_vram_gb must be finite and nonnegative");
        }
        if !self.max_ssd_gb.is_finite() || self.max_ssd_gb < 0.0 {
            return Err("max_ssd_gb must be finite and nonnegative");
        }
        if !(1..=100).contains(&self.cpu_target_percent) {
            return Err("cpu_target_percent must be between 1 and 100");
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub error: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_budget_boundaries() {
        assert!(Budget::default().validate().is_ok());
        assert!(Budget {
            max_ram_gb: 0.0,
            ..Budget::default()
        }
        .validate()
        .is_err());
        assert!(Budget {
            max_vram_gb: f64::NAN,
            ..Budget::default()
        }
        .validate()
        .is_err());
        assert!(Budget {
            cpu_target_percent: 101,
            ..Budget::default()
        }
        .validate()
        .is_err());
    }
}
