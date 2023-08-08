fn main() {
    println!("{:?}", recur_make_grades(&[-100,-10,0,1,20,30,40,50,60,70,80,90,100,110]));
}

fn make_grades(v: &[i64]) -> Vec<String>{
    let mut result = Vec::new();
    let mut i = 0;
    loop {
        if i != v.len().try_into().unwrap() {
            result.insert(i,
            match v[i] {
            0..=49 => format!("{}","Failed with F"),
            50..=60 => format!("{}","D"),
            61..=70 => format!("{}","C"),
            71..=80 => format!("{}","B"),
            81..=94 => format!("{}","A"),
            95..=100 => format!("{}","Excellent with A+"),
            _ => format!("{}","Invalid score"),
        });
         i += 1;
        } else {
            break;
        }
    }
    result
}

#[test]
fn test_grade() {
    let empty_array: [&str;0] = [];
    assert_eq!(make_grades(&[]), empty_array);
    assert_eq!(make_grades(&[0]), ["Failed with F"]);
    assert_eq!(make_grades(&[-100,-10,0,1,20,30,40,50,60,70,80,90,100,110]),&["Invalid score", "Invalid score", "Failed with F", "Failed with F", "Failed with F", "Failed with F", "Failed with F", "D", "D", "C", "B", "A", "Excellent with A+", "Invalid score"])
}

fn recur_make_grades(v: &[i64]) -> Vec<String>{
    let mut result = make_grades(&v[1..]);
    if v.is_empty() {
    } else {
    result.insert(0,
        match v[0] {
            0..=49 => format!("{}","Failed with F"),
            50..=60 => format!("{}","D"),
            61..=70 => format!("{}","C"),
            71..=80 => format!("{}","B"),
            81..=94 => format!("{}","A"),
            95..=100 => format!("{}","Excellent with A+"),
            _ => format!("{}","Invalid score"),
        });
    }
    result
}

#[test]
fn test_recur_grade() {
    let empty_array: [&str;0] = [];
    assert_eq!(recur_make_grades(&[]), empty_array);
    assert_eq!(recur_make_grades(&[0]), ["Failed with F"]);
    assert_eq!(recur_make_grades(&[-100,-10,0,1,20,30,40,50,60,70,80,90,100,110]),&["Invalid score", "Invalid score", "Failed with F", "Failed with F", "Failed with F", "Failed with F", "Failed with F", "D", "D", "C", "B", "A", "Excellent with A+", "Invalid score"])
}