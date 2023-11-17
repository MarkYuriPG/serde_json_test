use serde::{Deserialize, Serialize};
use serde_json::{to_string, from_str};
use std::fs::File;
use std::io::{self, Write};

//CRUD need
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Sources{
    source_files: Vec<String>,
    source_urls: Vec<String>,
    source_texts: Vec<TextFile>
}

//CRUD need
#[derive(Serialize,Deserialize, Debug, Clone)]
pub struct TextFile {
    pub title: String,
    pub content: String
}

#[derive(Serialize,Deserialize, Debug, Clone)]
struct Lesson{
    title: String,
    target_path: String,
    sources: Sources
}

//DATABASE CRUD NEED
#[derive(Serialize,Deserialize, Debug, Clone)]
struct Lessons{
    lessons: Vec<Lesson>
}

impl Lessons{
    fn create_lesson(&mut self, new_lesson: Lesson) {
        // Check for duplicate target_path before adding
        if !self.lessons.iter().any(|lesson| lesson.target_path == new_lesson.target_path) {
            self.lessons.push(new_lesson);
            println!("Lesson added successfully!");
        } else {
            println!("Error: Duplicate target_path found. Lesson not added.");
        }
    }

    fn remove_lesson(&mut self, target_path: &str) {
        if let Some(index) = self.lessons.iter().position(|lesson| lesson.target_path == target_path) {
            self.lessons.remove(index);
            println!("Lesson with target_path '{}' removed successfully!", target_path);
        } else {
            println!("Error: Lesson with target_path '{}' not found.", target_path);
        }
    }
}

fn write_lessons_to_file(lessons: &Lessons, file_path: &str) -> std::io::Result<()> {
    let json_string = serde_json::to_string_pretty(lessons)?;
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

fn main(){
    //ADD, no Duplicate target path
    let mut lessons_json = Lessons { lessons: Vec::new() };

    loop {
        println!("Choose Action:\n[1]Add Lesson\n[2]Remove Lesson\n[3]Exit");
        let mut input = String::new();
        let mut lesson_input = String::new();
        let mut file_path = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "1"
        {
            println!("Input lesson details in json format:");
            io::stdin()
            .read_line(&mut lesson_input)
            .expect("Failed to lesson_input");

            match serde_json::from_str::<Lesson>(&lesson_input.trim()) {
                Ok(new_lesson) => {
                    lessons_json.create_lesson(new_lesson);
                }
                Err(err) => {
                    eprintln!("Failed to parse JSON: {}", err);
                }
            }
        }
        else if input.trim() == "2"
        {
            println!("Input the file path of lesson to be deleted:");
            io::stdin()
            .read_line(&mut file_path)
            .expect("Failed to lesson_input");
            lessons_json.remove_lesson(file_path.trim());
        }
        else
        {
            break;
        }

        if let Err(err) = write_lessons_to_file(&lessons_json, "F:/SCHOOL/2023-2024/Lesson_JSON/lesson_data.json") {
            eprintln!("Failed to write to file: {}", err);
        }
    }
}
    
    // let cloned_lessons_data = lessons_json;

    // let json_string = serde_json::to_string_pretty(&cloned_lessons_data).expect("Failed to serialize data to JSON");

    // //WRITE the FILE only when remove or add happen
    // let mut file = File::create("F:/SCHOOL/2023-2024/Lesson_JSON/lesson_data.json").expect("Failed to create file");
    // file.write_all(json_string.as_bytes()).expect("Failed to write to file");


    // let mut lessons_json = Lessons{
    //     lessons: vec! [
            // Lesson {
            //     title: "lesson title".to_string(),
            //     target_path: "F:/SCHOOL/2023-2024/".to_string(),
            //     sources: Sources {
            //         source_files: vec![
            //             "path/to/file/1".to_string(),
            //             "path/to/file/2".to_string(),
            //             "path/to/file/3".to_string(),
            //         ],
            //         source_urls: vec![
            //             "url1".to_string(),
            //             "url2".to_string(),
            //             "url3".to_string(),
            //         ],
            //         source_texts: vec![
            //             TextFile {
            //                 title: "TextFile1".to_string(),
            //                 content: "Content of TextFile1".to_string(),
            //             },
            //             TextFile {
            //                 title: "TextFile2".to_string(),
            //                 content: "Content of TextFile2".to_string(),
            //             },
            //             TextFile {
            //                 title: "TextFile3".to_string(),
            //                 content: "Content of TextFile3".to_string(),
            //             },
            //         ],
            //     },
            // },
        // ]
    // };
    

    // lessons_json.create_lesson(
    //     Lesson {
    //         title: "lesson title3".to_string(),
    //         target_path: "F:/SCHOOL/2023-2024/Another_Lesson".to_string(),
    //         sources: Sources {
    //             source_files: vec![
    //                 "path/to/file/1".to_string(),
    //                 "path/to/file/2".to_string(),
    //                 "path/to/file/3".to_string(),
    //             ],
    //             source_urls: vec![
    //                 "url1".to_string(),
    //                 "url2".to_string(),
    //                 "url3".to_string(),
    //             ],
    //             source_texts: vec![
    //                 TextFile {
    //                     title: "TextFile1".to_string(),
    //                     content: "Content of TextFile1".to_string(),
    //                 },
    //                 TextFile {
    //                     title: "TextFile2".to_string(),
    //                     content: "Content of TextFile2".to_string(),
    //                 },
    //                 TextFile {
    //                     title: "TextFile3".to_string(),
    //                     content: "Content of TextFile3".to_string(),
    //                 },
    //             ],
    //         },
    //     },
    // );

    // lessons_json.remove_lesson("F:/SCHOOL/2023-2024/Another_Lesson");
//}