use std::cmp::Ordering;

#[derive(Debug)]
pub struct RatioTable<'a> {
    inner: Vec<RatioTableEntry<'a>>,
}

#[derive(Debug)]
pub(crate) struct RatioTableEntry<'a> {
    pub pallet: &'a str,
    pub extrinsic: &'a str,
    pub avg_extrinsic_time: f64,
    pub avg_storage_root_time: f64,
    pub ratio: f64,
    pub percentage: f64,
}

impl<'a> RatioTable<'a> {
    pub fn new() -> Self {
        RatioTable { inner: Vec::new() }
    }
    pub(crate) fn push(&mut self, entry: RatioTableEntry<'a>) {
        self.inner.push(entry);
    }
    /// Returns a list of the entries.
    ///
    /// Data ordered as:
    /// - pallet
    /// - extrinsic
    /// - average extrinsic time
    /// - average storage root time
    /// - ratio
    /// - percentage
    ///
    /// # Example output:
    /// ```
    /// vec![
    ///     ("identity", "add_registrar", 1.0, 0.0),
    ///     ("treasury", "tip_new", 1.8363, 83.6271),
    ///     ("balances", "transfer", 2.4501, 145.0108),
    /// ];
    /// ```
    pub fn raw_list(&self) -> Vec<(&str, &str, f64, f64, f64, f64)> {
        self.inner
            .iter()
            .map(|e| {
                (
                    e.pallet,
                    e.extrinsic,
                    e.avg_extrinsic_time,
                    e.avg_storage_root_time,
                    e.ratio,
                    e.percentage,
                )
            })
            .collect()
    }
    pub fn sort_by_ratio(&mut self) {
        self.inner
            .sort_by(|a, b| a.ratio.partial_cmp(&b.ratio).unwrap_or(Ordering::Equal));
    }
    pub fn print_entries(&self) {
        let width = 14;

        // Print table header
        println!(
            "|{:^width$}|{:^width$}|{:^width$}|{:^width$}|",
            "Pallet",
            "Extrinsic",
            "Ratio",
            "Increase",
            width = width
        );

        // Print line
        for _ in 0..4 {
            print!("|{:-<width$}", "", width = width);
        }
        println!("|");

        // Print table body
        for entry in &self.inner {
            println!(
                "|{:<width$}|{:<width$}|{:<width$}|{:>width_incr$} %|",
                entry.pallet,
                entry.extrinsic,
                entry.ratio,
                entry.percentage,
                width = width,
                width_incr = width - 2
            );
        }
    }
}

#[derive(Debug)]
pub struct StepIncrTable<'a> {
    inner: Vec<StepIncrTableEntry<'a>>,
}

#[derive(Debug, Default)]
pub(crate) struct StepIncrTableEntry<'a> {
    pub pallet: &'a str,
    pub extrinsic: &'a str,
    pub steps: Vec<StepRepeatIncr<'a>>,
}

#[derive(Debug)]
pub(crate) struct StepRepeatIncr<'a> {
    pub input_vars: &'a Vec<u64>,
    pub avg_extrinsic_time: f64,
    pub avg_storage_root_time: f64,
    pub extrinsic_percentage: f64,
    pub storage_root_percentage: f64,
}

impl<'a> StepIncrTable<'a> {
    pub fn new() -> Self {
        StepIncrTable { inner: Vec::new() }
    }
    pub(crate) fn push(&mut self, entry: StepIncrTableEntry<'a>) {
        self.inner.push(entry);
    }
    pub fn sort_by_extrinsic_percentage(&mut self) {
        for entry in &mut self.inner {
            entry.steps.sort_by(|a, b| {
                b.extrinsic_percentage
                    .partial_cmp(&a.extrinsic_percentage)
                    .unwrap_or(Ordering::Equal)
            });
        }
    }
    /// Returns a list of the entries.
    ///
    /// Data ordered as:
    /// - pallet
    /// - extrinsic
    /// - input variables
    /// - average extrinsic time
    /// - average storage root time
    /// - percentage increase of extrinsic time compared to the lowest
    /// - percentage increase of storage root time compared to the lowest
    ///
    /// # Example output:
    /// ```
    /// vec![
    ///     ("balances", "transfer", [892, 1000], 194126.4, 90757.4, 8.4298, 29.2032)
    ///     ("balances", "transfer", [298, 1000], 190419.6, 87388.7, 6.3594, 24.4075)
    ///     ("balances", "transfer", [397, 1000], 187451.3, 79826.0, 4.7014, 13.6412)
    /// ];
    /// ```
    pub fn raw_list(&self) -> Vec<(&str, &str, &[u64], f64, f64, f64, f64)> {
        self.inner
            .iter()
            .map(|e| {
                e.steps
                    .iter()
                    .map(|s| {
                        (
                            e.pallet,
                            e.extrinsic,
                            s.input_vars.as_slice(),
                            s.avg_extrinsic_time,
                            s.avg_storage_root_time,
                            s.extrinsic_percentage,
                            s.storage_root_percentage,
                        )
                    })
                    .collect::<Vec<(&str, &str, &[u64], f64, f64, f64, f64)>>()
            })
            .flatten()
            .collect()
    }
}
