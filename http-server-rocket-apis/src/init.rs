use rocket::post;
use crate::state::*;

#[post("/init")]
pub async fn init_server() -> Option<String> {
    // we are using unsafe block because modifying static variables
    // it's because - multiple threads can use static variable simultaneously which is unsafe.
    unsafe {
        STUDENTS.push(StudentInfo::create_student(1, "naidu", 30, GENDER::MALE));
        STUDENTS.push(StudentInfo::create_student(2, "grace", 25, GENDER::FEMALE));

        TEACHERS.push(TeacherInfo::create_teacher(1, "akash", 45, GENDER::MALE, &vec![SUBJECTS::SCIENCE, SUBJECTS::BIOLOGY]));
        TEACHERS.push(TeacherInfo::create_teacher(2, "preeti", 53, GENDER::FEMALE, &vec![SUBJECTS::SCIENCE, SUBJECTS::BIOLOGY]));

        STUDENT_COUNT += 2;
        TEACHER_COUNT += 2;
    }

    Some(format!("Server init - Success"))
}
