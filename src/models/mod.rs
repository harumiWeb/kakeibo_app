use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum IncomeCategory {
    Salary,
    Bonus,
    Other,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ExpenseCategory {
    Food,
    Hobby,
    Other,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Category {
    Income(IncomeCategory),
    Expense(ExpenseCategory),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Item {
    name: String,
    category: Category,
    price: u32,
    date: NaiveDate,
}

impl Item {
    pub fn new(name: String, category: Category, price: u32, date: NaiveDate) -> Self {
        Self {
            name,
            category,
            price,
            date,
        }
    }

    pub fn get_category(register_type: u8, category_type: u8) -> Category {
        if register_type == 0 {
            match category_type {
                0 => Category::Income(IncomeCategory::Salary),
                1 => Category::Income(IncomeCategory::Bonus),
                2 => Category::Income(IncomeCategory::Other),
                _ => panic!("カテゴリの入力値が不正です。"),
            }
        } else {
            match category_type {
                0 => Category::Expense(ExpenseCategory::Food),
                1 => Category::Expense(ExpenseCategory::Hobby),
                2 => Category::Expense(ExpenseCategory::Other),
                _ => panic!("カテゴリの入力値が不正です。"),
            }
        }
    }

    pub fn get_year(&self) -> i32 {
        self.date.year()
    }

    pub fn get_month(&self) -> u32 {
        self.date.month()
    }

    pub fn get_first_day(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.get_year(), self.get_month(), 1).unwrap()
    }

    pub fn get_price_for_summary(&self) -> i32 {
        match self.category {
            Category::Income(_) => self.price as i32,
            Category::Expense(_) => -1 * self.price as i32,
        }
    }
}
