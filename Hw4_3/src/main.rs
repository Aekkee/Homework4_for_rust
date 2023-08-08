fn main() {
    println!("{}", ar2(&[-10]));
}

fn ar1(v: &[i64]) -> String {
    let mut result = String::new();
    let mut i = 0; 
    if v.is_empty() {
    } else {
        while i < v[0] * 2{
            if i <= v[0] {
                result.push_str(&format!("{}\n", "*".repeat(i.try_into().unwrap())));
            } else {
                result.push_str(&format!("{}\n", "*".repeat((2 * v[0] - i).try_into().unwrap())));
            }
            i += 1;
        }
    }
    result
}

fn ar2(v: &[i64]) -> String {
    let mut result = String::new();
    let mut i = 0; 
    if v.is_empty() {
    } else {
        while i < v[0] * 2{
            if i <= v[0] {
                result.push_str(&format!(
                    "{} {}\n",
                    " ".repeat((v[0] - i).try_into().unwrap()),
                    "*".repeat(i.try_into().unwrap())
                ));
            } else if i > v[0] {
                result.push_str(&format!(
                    "{}{}\n",
                    " ".repeat((i - v[0] +1).try_into().unwrap()),
                    "*".repeat((2 * v[0] - i).try_into().unwrap())
                ));
            } 
        i += 1;
        }
    }
    result
}

#[test]
fn test_ar1() {
    assert_eq!(ar1(&[]), "");
    assert_eq!(ar1(&[0]), "");
    assert_eq!(ar1(&[-1]), "");
    assert_eq!(ar1(&[4]), "\n*\n**\n***\n****\n***\n**\n*\n");
    assert_eq!(ar1(&[7]), "\n*\n**\n***\n****\n*****\n******\n*******\n******\n*****\n****\n***\n**\n*\n");
}

#[test]
fn test_ar2() {
    assert_eq!(ar2(&[]), "");
    assert_eq!(ar2(&[0]), "");
    assert_eq!(ar2(&[-1]), "");
    assert_eq!(ar2(&[4]), "     \n    *\n   **\n  ***\n ****\n  ***\n   **\n    *\n");
    assert_eq!(ar2(&[7]), "        \n       *\n      **\n     ***\n    ****\n   *****\n  ******\n *******\n  ******\n   *****\n    ****\n     ***\n      **\n       *\n");
}


fn recur_ar1(v: &[i64]) -> String {
    let mut result = String::new();
    if v.is_empty() {
    } else {
        let r = v[0];        
        fn inner_recur_ar1(i: i64,r: i64, result: &mut String){
            if i < r * 2{
            if i <= r {
                result.push_str(&format!("{}\n", "*".repeat(i.try_into().unwrap())));
            } else {
                result.push_str(&format!("{}\n", "*".repeat((2 * r - i).try_into().unwrap())));
            }
        inner_recur_ar1(i+1, r, result);
        }
    }
            inner_recur_ar1( 0, r, &mut result);
    }
    result
}

fn recur_ar2(v: &[i64]) -> String {
    let mut result = String::new();
    if v.is_empty() {
    } else {
        let r = v[0];        
        fn inner_recur_ar1(i: i64,r: i64, result: &mut String){
            if i < r * 2 {
            if i <= r {
                result.push_str(&format!(
                    "{} {}\n",
                    " ".repeat((r - i).try_into().unwrap()),
                    "*".repeat(i.try_into().unwrap())
                ));
            } else if i > r {
                result.push_str(&format!(
                    "{}{}\n",
                    " ".repeat((i - r + 1).try_into().unwrap()),
                    "*".repeat((2 * r - i).try_into().unwrap())
                ));
            } 
        inner_recur_ar1(i+1, r, result);
            }
        }
            inner_recur_ar1( 0, r, &mut result);
    }
    result
}

#[test]
fn recur_test_ar1() {
    assert_eq!(ar1(&[]), "");
    assert_eq!(ar1(&[0]), "");
    assert_eq!(ar1(&[-1]), "");
    assert_eq!(ar1(&[4]), "\n*\n**\n***\n****\n***\n**\n*\n");
    assert_eq!(ar1(&[7]), "\n*\n**\n***\n****\n*****\n******\n*******\n******\n*****\n****\n***\n**\n*\n");
}

#[test]
fn recur_test_ar2() {
    assert_eq!(ar2(&[]), "");
    assert_eq!(ar2(&[0]), "");
    assert_eq!(ar2(&[-1]), "");
    assert_eq!(ar2(&[4]), "     \n    *\n   **\n  ***\n ****\n  ***\n   **\n    *\n");
    assert_eq!(ar2(&[7]), "        \n       *\n      **\n     ***\n    ****\n   *****\n  ******\n *******\n  ******\n   *****\n    ****\n     ***\n      **\n       *\n");
}
