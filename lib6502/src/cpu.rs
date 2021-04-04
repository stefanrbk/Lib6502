impl super::Cpu {
    fn cycle(&self) {
        for f in &self.phase_1_positive_edge {
            f();
        }
        for f in &self.phase_1_negative_edge {
            f();
        }
        for f in &self.phase_2_positive_edge {
            f();
        }
        for f in &self.phase_2_negative_edge {
            f();
        }
    }
}
