#[derive(Debug, Clone)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}
impl RiskLevel {
    pub fn symbol(&self) -> &str {
        match self {
            RiskLevel::Critical => "⚠️",
            RiskLevel::High => "🔴",
            RiskLevel::Medium => "🟡",
            RiskLevel::Low => "🟢",
        }
    }
    pub fn label(&self) -> &str {
        match self {
            RiskLevel::Critical => "CRITICAL",
            RiskLevel::High => "HIGH",
            RiskLevel::Medium => "MEDIUM",
            RiskLevel::Low => "LOW",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub repo: String,
    pub current_version: String,
    pub new_version: String,
    pub architecture: String,
    pub risk_level: RiskLevel,
    pub selected:bool,
}
