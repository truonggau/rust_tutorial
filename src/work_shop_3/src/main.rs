use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut schools = School::new();
    println!("schools: {:?}", schools.students);
    schools.add(7, "Alice");
    schools.add(10, "Bob");
    schools.add(7, "Charlie");
    println!("schools: {:?}", schools.students);
    let grade_7 = schools.grade(7);
    println!("grades 7: {:?}", grade_7);
    let grades = schools.grades();
    println!("grades: {:?}", grades);
}


// Bài tập
// Cho ngữ cảnh như sau : Một ngôi trường cần lập danh
//sách thông tin sinh viên bao gồm tên sinh viên và điểm của sinh viên đó.
// với mục đích thống kê kiểm tra giáo dục của ngôi trường này


/*-----------------------------*/
// Gợi ý:
// Định nghĩa bằng struct, mọi người nên sử dụng HashMap 
// Tại sao lại sử dụng HashMap và ko phải Vec
//https://doc.rust-lang.org/std/collections/struct.HashMap.html
// struct School {
//     students: HashMap<String, u32>
// }

// trong trường hợp này thì String : tên của sinh viên đó
// u32 là điểm số 

// Một số yêu cầu như sau:

/*-----------------------------*/
//0. Tạo 1 function new() khởi tạo rỗng ban đầu cho danh sách

/*-----------------------------*/
//1. Có thể thêm thông tin của sinh viên gồm có tên và điểm
// Ví dụ: thêm tên "Alice" có 7 điểm, thêm tên "Bob" có 2 điểm, ...
// Gợi ý : định nghĩa hàm "add" implement cho Struct

/*-----------------------------*/
//2. Liệt kê các điểm số hiện tại mà trường đã cập nhập
// ví dụ :danh sách hiện tại gồm có [(Alice, 10), (Bob,4)]
//trả về là [4,10] (điểm số nên tăng dần và ko có duplicate)
// ví dụ: [(Alice, 10), (Bob,4), (Steve,4)] -> [4,10]

/*-----------------------------*/
//3. Liệt kê danh sách các học sinh có cùng 1 điểm số
// ví dụ: hiện tại danh sách gồm có (Alice, 3), (Bob, 10), (Charlie,3)
// liệt kê danh sách học sinh có cùng 3 điểm : [Alice, Charlie]

// Yêu cầu trả về: danh sách sinh viên là alphabet theo tên 

// Gợi ý: 
// ví dụ : Ban đầu [(Alice, 10), (Bob,2), (Eve,4), (Long,2)] -> [(Bob, 2), (Long,2), (Eve,4), (Alice,10)]
//định nghĩa hàm "find_student" có tham số là điểm số -> trả về danh sách các sinh viên có cùng điểm số mong muốn


// Các bạn phải vuợt qua một số test case như sau :

/*-----------------------------*/
//Test case 1: Khởi tạo đầu tiên danh sách phải rỗng

/*-----------------------------*/

// Test case 2:
//Thêm sinh viên có tên "Lee" với điểm số là 2
// thì tất cả các điểm số hiện có của trường là 2
//nếu thêm sinh viên khác "Nancy" với điểm số là 3
//thì các điểm số hiện tại là [2,3]

/*-----------------------------*/

// Test case 3:
// Giả sử danh sách hiện tại : [(Bob, 4), (Alice,4), (Tom,5)]
// với điểm số 4 thì ta có sinh viên nào: -> [Alice, Bob] not [Bob ,Alice]
// vì cần tên theo alphabet


/*-----------------------------*/
// Nếu mọi người làm xong rùi thì có thể làm advance hơn bằng cách 
// sử dụng Generic type cho điểm số không nhất thiết phải U32 nữa mà có thể "A+", "B+" chẳng hạn (string)
/*-----------------------------*/

// Sườn thông tin cho mọi người dễ làm



pub struct School {
    students: HashMap<String, u32>
}

impl School {
    pub fn new() -> School {
        return School{students: HashMap::new()};
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(student.parse().unwrap(), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = Vec::new();
        for i in self.students.values() {
            if !grades.contains(i) {
                grades.push(*i)
            }
        }
        grades.sort();
        return grades;

    }


    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut student_name = Vec::new();
        for i in self.students.keys() {
            if grade == self.students[i] {
                student_name.push(i.clone())
            }
        }

        return student_name;
    }
}
