use quarters;

#[derive(Debug)]
pub struct Sett {
    pub age: i32,
    pub pop: i32,
    pub gold: f64,
    pub qrtrs: Vec<Quarter>,
}

impl Sett {
    /// Execute settlement timestep
    pub fn step(&self) {
        // call each quarter's step
        // accumulate gold
        unimplemented!()
    }

    /// Add quarter
    /// Gain a small amount of gold for doing so.
    /// Move a fraction of the sett's population to the new quarter,
    /// times the growth bonus.
    pub fn add_quarter(&self, qc: quarters::QuarterConf)
    -> Result<Self, quarters::BuildErr>
    {
        // ensure pop is high enough
        // remove pop from existing quarters equally
        // multiply number by growth bonus => newpop
        // call quarter::Quarter::new(n, qt, newpop, r);
        // receive gold bonus
        unimplemented!()
    }
}
