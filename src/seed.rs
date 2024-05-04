use rusqlite::{Connection, Result};
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use rocket::serde::json::serde_json;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MuscleGroup {
    name: String,
    body_part: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exercise {
    id: i32,
    main_muscle_group: String,
    long_description: String,
    importance: f32,
    name: String,
    description: String,
    long: String,
    user_id: i32,
}

fn load_defaults() -> (Vec<MuscleGroup>, Vec<Exercise>) {
    let muscle_groups_file_path = Path::new("./muscle_groups.json");
    let workout_lists_file_path = Path::new("./workout_list.json");

    let file = File::open(muscle_groups_file_path).expect("error while opening");

    let muscle_groups: Vec<MuscleGroup> = serde_json::from_reader(&file)
    .expect("error while reading or parsing");

    drop(file);

    let file2 = File::open(workout_lists_file_path).expect("error while opening");

    let workout_list: Vec<Exercise> = serde_json::from_reader(&file2)
    .expect("error while reading or parsing");

    drop(file2);

    return (muscle_groups, workout_list);
}

pub fn seed_db() -> Result<()> {
    let conn = Connection::open("theSplit.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS muscle_group (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            body_part TEXT NOT NULL
        )",
        (),
    )?;
    
    conn.execute(
        "create TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT,
            last_name TEXT,
            email TEXT NOT NULL
          )",
        (),
    )?;

    conn.execute(
        "create TABLE IF NOT EXISTS exercise (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
              name TEXT NOT NULL,
              importance INTEGER NOT NULL,
              main_muscle_group TEXT NOT NULL,
              description TEXT NOT NULL,
              long_description TEXT,
              user_id INTEGER,
              FOREIGN KEY (user_id) REFERENCES user(id),
              FOREIGN KEY (main_muscle_group) REFERENCES muscle_group(name)
          )",
        (),
    )?;

    let defaults = load_defaults();

    for muscle_group in &defaults.0 {
        conn.execute(
            "INSERT INTO muscle_group (name, body_part) values (?1, ?2)",
            &[&muscle_group.name, &muscle_group.body_part],
        )?;
    }

    for exercise in &defaults.1 {
        conn.execute(
            "INSERT INTO muscle_group (name, importance, main_muscle_group, description, long_description, user_id) values (?1, ?2, ?3, ?4, ?5, ?6)",
            &[&exercise.name, &exercise.importance.to_string(), &exercise.main_muscle_group, &exercise.description, &exercise.long_description, &exercise.user_id.to_string()],
        )?;
    }

    Ok(())
}
