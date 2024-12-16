use crate::{models, services};
use chrono::{NaiveDate, Datelike};
use std::collections::{BTreeMap, BTreeSet};

pub fn run(file_path: &str) {
  println!("家計簿の集計を行います");
  let data = services::io::read_data_or_panic(file_path);

  let target_dates: BTreeSet<NaiveDate> = get_target_dates(&data);
  let mut result_table: BTreeMap<NaiveDate, i32> = BTreeMap::new();

  for date in target_dates {
    let filtered_data = get_filtered_data(&data, date);
    let sum = summarize_data(&filtered_data);
    result_table.insert(date, sum);
  }
  print_table(result_table);
}

fn get_target_dates(data: &Vec<models::Item>) -> BTreeSet<NaiveDate> {
    let target_dates: BTreeSet<_> = data
        .iter()
        .map(|item: &models::Item| item.get_first_day())
        .collect();
    target_dates
}

fn get_filtered_data(data: &Vec<models::Item>, filter_date: NaiveDate) -> Vec<&models::Item> {
  let filter_data: Vec<&models::Item> = data.iter().filter(|item| {
    (item.get_year() == filter_date.year()) && (item.get_month() == filter_date.month())
  }).collect();
  filter_data
}

fn summarize_data(data: &Vec<&models::Item>) -> i32 {
  let mut sum = 0;
  for item in data {
    sum += item.get_price_for_summary();
  }
  sum
}

fn format_date(date: NaiveDate) -> String {
  format!("{}/{}", date.year(), date.month())
}

fn format_price(price: i32) -> String {
  if price >= 0 {
    format!("+{}", price)
  } else {
    format!("{}", price)
  }
}

fn print_table(result_table: BTreeMap<NaiveDate, i32>) {
  for result in result_table {
    let date = format_date(result.0);
    let price = format_price(result.1);
    println!("{}の収支は{}円でした", date, price);
  }
}

#[cfg(test)]
mod summarize_test {
  use super::*;

  #[test]
  fn test_get_target_dates() {
    let data = get_test_data();
    let target_dates = get_target_dates(&data);
    assert_eq!(target_dates, BTreeSet::from([NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()]));
  }

  fn get_test_data() -> Vec<models::Item> {
    let data = vec![
      models::Item::new("test1".to_string(), models::Category::Income(models::IncomeCategory::Salary), 1000, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
      models::Item::new("test2".to_string(), models::Category::Income(models::IncomeCategory::Salary), 2000, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
      models::Item::new("test3".to_string(), models::Category::Income(models::IncomeCategory::Salary), 3000, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
    ];
    data
  }
}