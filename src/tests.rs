#[cfg(test)]
use chrono::prelude::Utc;
#[cfg(test)]
use chrono::TimeZone;

#[cfg(test)]
use crate::risk_free_model;

use test_log;

#[test_log::test]
fn one_year_test() {
    let rfm = risk_free_model::get_annualised_risk_free_rate(5.0);
    let start: f64 = 100.0;
    let begin_date = Utc.timestamp_millis_opt(1688917143000).unwrap();
    let end_date = Utc.timestamp_millis_opt(1720539543000).unwrap();
    let ret = rfm.apply(start, begin_date, end_date);
    assert_eq!(ret, 105.0);
}
