use rusqlite::{Connection, Result};
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use rocket::serde::json::serde_json;
use std::io::BufReader;
use std::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MuscleGroup {
    name: String,
    body_part: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Exercise {
    id: i32,
    muscle_group: Vec<String>,
    importance: f32,
    name: String,
    description: String,
    long: String,
}

//Result<User, Box<dyn Error>>

fn load_workout_list() -> Result<Vec<Exercise>, Box<dyn Error>> {
    let workout_lists_file_path = Path::new("./src/workout_list.json");
    let file2 = File::open(workout_lists_file_path)?;
    let reader2 = BufReader::new(&file2);

    let workout_list = serde_json::from_reader(reader2)?;

    Ok(workout_list)
}

fn load_muscle_groups() -> Result<Vec<MuscleGroup>, Box<dyn Error>> {
    let muscle_groups_file_path = Path::new("./src/muscle_groups.json");
    

    let file = File::open(muscle_groups_file_path)?;
    let reader = BufReader::new(&file);

    let muscle_groups = serde_json::from_reader(reader)?;

    Ok(muscle_groups)
}

pub fn seed_db() -> Result<()> {
    let conn = Connection::open("theSplit.db")?;

    conn.execute(
        "DROP TABLE IF EXISTS muscle_group",()
    )?;
    conn.execute(
        "DROP TABLE IF EXISTS user",()
    )?;
    conn.execute(
        "DROP TABLE IF EXISTS exercise",()
    )?;

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

    let muscle_groups = load_muscle_groups().unwrap();
    let exercises = load_workout_list().unwrap();

    println!("{:#?}", muscle_groups);
    println!("{:#?}", exercises);

    for muscle_group in &muscle_groups {
        conn.execute(
            "INSERT INTO muscle_group (name, body_part) values (?1, ?2)",
            &[&muscle_group.name, &muscle_group.body_part],
        )?;
    }

    for exercise in &exercises {
        conn.execute(
            "INSERT INTO muscle_group (name, importance, main_muscle_group, description, long_description, user_id) values (?1, ?2, ?3, ?4, ?5, ?6)",
            &[&exercise.name, &exercise.importance.to_string(), &exercise.muscle_group[0], &exercise.description, &exercise.long, 0.to_string()],
        )?;
    }

    Ok(())
}
