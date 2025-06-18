use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Student {
    id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AttendanceRecord {
    date: String,
    status: bool, // true for present, false for absent
}

#[derive(Debug, Serialize, Deserialize)]
struct AttendanceSystem {
    students: HashMap<u32, Student>,
    records: HashMap<u32, Vec<AttendanceRecord>>, // student_id -> records
}

impl AttendanceSystem {
    fn new() -> Self {
        AttendanceSystem {
            students: HashMap::new(),
            records: HashMap::new(),
        }
    }

    fn add_student(&mut self, id: u32, name: String) {
        let student = Student { id, name };
        self.students.insert(id, student);
        self.records.insert(id, Vec::new());
        println!("Student added successfully!");
    }

    fn mark_attendance(&mut self, student_id: u32, date: String, status: bool) {
        if let Some(records) = self.records.get_mut(&student_id) {
            let record = AttendanceRecord { date, status };
            records.push(record);
            println!("Attendance marked successfully!");
        } else {
            println!("Student not found!");
        }
    }

    fn view_student_attendance(&self, student_id: u32) {
        if let Some(student) = self.students.get(&student_id) {
            println!("Attendance for {} (ID: {}):", student.name, student.id);
            
            if let Some(records) = self.records.get(&student_id) {
                for record in records {
                    let status = if record.status { "Present" } else { "Absent" };
                    println!("Date: {}, Status: {}", record.date, status);
                }
            }
        } else {
            println!("Student not found!");
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let data = serde_json::to_string(self)?;
        fs::write(filename, data)?;
        println!("Data saved to {} successfully!", filename);
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        let data = fs::read_to_string(filename)?;
        let system: AttendanceSystem = serde_json::from_str(&data)?;
        println!("Data loaded from {} successfully!", filename);
        Ok(system)
    }
}

fn main() {
    let mut system = if Path::new("attendance.json").exists() {
        AttendanceSystem::load_from_file("attendance.json").unwrap_or_else(|_| {
            println!("Error loading file, creating new system.");
            AttendanceSystem::new()
        })
    } else {
        AttendanceSystem::new()
    };

    loop {
        println!("\nAttendance System Menu:");
        println!("1. Add Student");
        println!("2. Mark Attendance");
        println!("3. View Student Attendance");
        println!("4. Save Data");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter student ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read input");
                let id: u32 = id.trim().parse().expect("Please enter a number");

                println!("Enter student name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read input");
                let name = name.trim().to_string();

                system.add_student(id, name);
            }
            "2" => {
                println!("Enter student ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read input");
                let id: u32 = id.trim().parse().expect("Please enter a number");

                println!("Enter date (YYYY-MM-DD):");
                let mut date = String::new();
                io::stdin().read_line(&mut date).expect("Failed to read input");
                let date = date.trim().to_string();

                println!("Is student present? (y/n):");
                let mut status = String::new();
                io::stdin().read_line(&mut status).expect("Failed to read input");
                let status = status.trim().to_lowercase() == "y";

                system.mark_attendance(id, date, status);
            }
            "3" => {
                println!("Enter student ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read input");
                let id: u32 = id.trim().parse().expect("Please enter a number");

                system.view_student_attendance(id);
            }
            "4" => {
                if let Err(e) = system.save_to_file("attendance.json") {
                    println!("Error saving file: {}", e);
                }
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}