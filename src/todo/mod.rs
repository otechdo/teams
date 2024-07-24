use chrono::{Datelike, Duration, Local, NaiveDate};

pub const TODO_TABLE_CREATION: &str = "CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    start_date TEXT DEFAULT CURRENT_TIMESTAMP,
    due_date TEXT DEFAULT CURRENT_TIMESTAMP,
    priority INTEGER NOT NULL,
    completed INTEGER DEFAULT 0, 
    state INTEGER DEFAULT 0, 
    assignee_id INTEGER NOT NULL CHECK (assignee_id != 1),  
    root_manager_id INTEGER DEFAULT 1, 
    FOREIGN KEY(assignee_id) REFERENCES users(id),
    FOREIGN KEY(root_manager_id) REFERENCES users(id)
);";

pub const USERS_TABLE_CREATION: &str = "CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('root_manager', 'project_manager', 'team_member'))
);";

pub const PROJECTS_TABLE_CREATION: &str = "CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    start_date TEXT DEFAULT CURRENT_TIMESTAMP,
    due_date TEXT,
    status TEXT NOT NULL CHECK (status IN (0, 1, 2, 3,4))
);";

pub const PROJECTS_TASK_TABLE_CREATION: &str = "CREATE TABLE IF NOT EXISTS project_tasks (
    project_id INTEGER,
    task_id INTEGER,
    FOREIGN KEY(project_id) REFERENCES projects(id),
    FOREIGN KEY(task_id) REFERENCES tasks(id),
    PRIMARY KEY (project_id, task_id)
);";

pub const ROOT_MANAGER_ID: i32 = 1;

pub const ASSIGN_TASK_TO: &str = "";
pub const DEFINE_TASK_CLOSE: &str = "";
pub const SEND_TASK_TO_ADMIN: &str = "";

pub enum TaskState {
    NotStarted, // Default when a task is created
    InProgress, // Work has begun on the task
    Blocked,    // Something is preventing progress
    Completed,  // Task has been finished
    Canceled,   // Task is no longer relevant
}

impl TaskState {
    pub const TASK_NOT_STARTED: i32 = 0;
    pub const TASK_IN_PROGRESS: i32 = 1;
    pub const TASK_BLOCKED: i32 = 2;
    pub const TASK_COMPLETED: i32 = 3;
    pub const TASK_CANCELED: i32 = 4;
}

pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl Quarter {
    #[must_use]
    pub fn min_month(self) -> String {
        match self {
            Self::Q1 => String::from("January"),
            Self::Q2 => String::from("April"),
            Self::Q3 => String::from("July"),
            Self::Q4 => String::from("October"),
        }
    }
    #[must_use]
    pub fn max_month(self) -> String {
        match self {
            Self::Q1 => String::from("March"),
            Self::Q2 => String::from("June"),
            Self::Q3 => String::from("September"),
            Self::Q4 => String::from("December"),
        }
    }
    #[must_use]
    pub fn middle_month(self) -> String {
        match self {
            Self::Q1 => String::from("February"),
            Self::Q2 => String::from("May"),
            Self::Q3 => String::from("August"),
            Self::Q4 => String::from("November"),
        }
    }
}
pub enum DueDate {
    Today,
    Tomorrow,
    InOneWeek,
    InOneMonth,
    Q1,
    Q2,
    Q3,
    Q4,
    Custom(NaiveDate),
}

impl DueDate {
    pub const TODAY: &str = "today";
    pub const TOMORROW: &str = "tomorrow";
    pub const IN_ONE_WEEK: &str = "in_one_week";
    pub const IN_ONE_MONTH: &str = "in_one_month";
    pub const Q1: &str = "Q1";
    pub const Q2: &str = "Q2";
    pub const Q3: &str = "Q3";
    pub const Q4: &str = "Q4";
    pub const CUSTOM: &str = "custom";

    #[must_use]
    pub fn current_quarter() -> Self {
        let now = Local::now();
        match now.month() {
            1..=3 => Self::Q1,
            4..=6 => Self::Q2,
            7..=9 => Self::Q3,
            10..=12 => Self::Q4,
            _ => unreachable!(),
        }
    }

    ///
    /// # Panics
    ///
    #[must_use]
    pub fn time_remaining(&self) -> (i64, i64, i64) {
        let today = Local::now().naive_local().date();
        let due_date = match self {
            Self::Today => today,
            Self::Tomorrow => today + Duration::days(1),
            Self::InOneWeek => today + Duration::weeks(1),
            Self::InOneMonth => {
                let mut next_month = today.with_month(today.month() + 1).unwrap_or(today);
                next_month = next_month.with_day(today.day()).expect("msg");
                next_month
            }
            Self::Q1 => NaiveDate::from_ymd_opt(today.year(), 3, 31).unwrap(),
            Self::Q2 => NaiveDate::from_ymd_opt(today.year(), 6, 30).unwrap(),
            Self::Q3 => NaiveDate::from_ymd_opt(today.year(), 9, 30).unwrap(),
            Self::Q4 => NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap(),
            Self::Custom(date) => *date,
        };

        let duration = due_date.signed_duration_since(today);
        let months = duration.num_days() / 30;
        let weeks = (duration.num_days() % 30) / 7;
        let days = duration.num_days() % 7;
        (months, weeks, days)
    }

    pub fn print_remaining(self) {
        let (months, weeks, days) = self.time_remaining();

        if months.is_negative() || weeks.is_negative() || days.is_negative() {
            println!(
                "{}",
                format_args!(
                    "Task is overdue by : {} months, {} weeks, {} days",
                    months.abs(),
                    weeks.abs(),
                    days.abs()
                )
            );
        } else {
            println!(
                "{}",
                format_args!(
                    "Time remaining : {} months, {} weeks, {} days",
                    months, weeks, days
                )
            );
        }
    }
}
