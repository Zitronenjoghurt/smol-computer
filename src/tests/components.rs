use crate::components::Component;
use crate::truth_table::TruthTable;
use std::fmt;

mod logic;

pub type ComponentTestResult<C> = Result<(), ComponentTestReport<C>>;

pub fn test_truth_table<C: Component>(
    component: &mut C,
    truth_table: TruthTable<C::Input, C::Output>,
) -> ComponentTestResult<C> {
    let mut report = ComponentTestReport::new(truth_table.entries.len());

    for (input, expected_output) in truth_table.entries {
        component.process(input);
        let actual_output = component.output();

        if actual_output != expected_output {
            report.add_failed_case(input, expected_output, actual_output);
        }
    }

    if report.passed() { Ok(()) } else { Err(report) }
}

#[macro_export]
macro_rules! assert_component_test {
    ($component:expr, $truth_table:expr) => {
        match $crate::tests::components::test_truth_table(&mut $component, $truth_table) {
            Ok(_) => (),
            Err(report) => {
                panic!("Component test failed:\n{}", report);
            }
        }
    };
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ComponentTestReport<C: Component> {
    pub cases_count: usize,
    pub failed_cases: Vec<ComponentTestFailedCase<C>>,
}

impl<C: Component> ComponentTestReport<C> {
    pub fn new(cases_count: usize) -> Self {
        Self {
            cases_count,
            failed_cases: Vec::new(),
        }
    }

    pub fn add_failed_case(
        &mut self,
        input: C::Input,
        expected_output: C::Output,
        actual_output: C::Output,
    ) {
        self.failed_cases.push(ComponentTestFailedCase {
            input,
            expected_output,
            actual_output,
        });
    }

    pub fn passed(&self) -> bool {
        self.failed_cases.is_empty()
    }

    pub fn success_rate(&self) -> f64 {
        if self.cases_count == 0 {
            return 1.0;
        }
        let successful_cases = self.cases_count - self.failed_cases.len();
        (successful_cases as f64) / (self.cases_count as f64)
    }
}

impl<C: Component> fmt::Display for ComponentTestReport<C>
where
    C::Input: fmt::Debug,
    C::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Component Test Report:")?;
        writeln!(f, "  Total test cases: {}", self.cases_count)?;
        writeln!(
            f,
            "  Passed test cases: {}",
            self.cases_count - self.failed_cases.len()
        )?;
        writeln!(f, "  Failed test cases: {}", self.failed_cases.len())?;
        writeln!(f, "  Success rate: {:.2}%", self.success_rate() * 100.0)?;

        if !self.failed_cases.is_empty() {
            writeln!(f, "\nFailed cases:")?;
            for (i, failed_case) in self.failed_cases.iter().enumerate() {
                writeln!(f, "Case #{}:", i + 1)?;
                writeln!(f, "{}", failed_case)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ComponentTestFailedCase<C: Component> {
    pub input: C::Input,
    pub expected_output: C::Output,
    pub actual_output: C::Output,
}

impl<C: Component> fmt::Display for ComponentTestFailedCase<C>
where
    C::Input: fmt::Debug,
    C::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Input:           {:?}", self.input)?;
        writeln!(f, "  Expected output: {:?}", self.expected_output)?;
        writeln!(f, "  Actual output:   {:?}", self.actual_output)
    }
}
