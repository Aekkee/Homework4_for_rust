fn main() {
    println!("{}", recur_fahr_to_cel_v(&[]));
}

fn fahr_to_cel_v(v: &[i64]) -> String {
    let mut result = String::new();
    result.push_str(&format!("{}\t{}\n", "Fahr", "Celcius"));
    if v.is_empty() {
    } else {
        let strP: i64 = v[0];
        let endP: i64 = v[1];
        let d: i64 = v[2];
        let mut count = 0i64;
        if v[2] == 0 {
            result.push_str(&format!(
                "{}\t{:.1}\n",
                strP + count * d,
                5.0 / 9.0 * ((strP as f64 + count as f64 * d as f64) - 32.0)
            ));
        } else {
            loop {
                if count != ((strP - endP).abs() / d) + 1 {
                    if strP < endP {
                        result.push_str(&format!(
                            "{}\t{:.1}\n",
                            strP + count * d,
                            5.0 / 9.0 * ((strP as f64 + count as f64 * d as f64) - 32.0)
                        ));
                    } else {
                        result.push_str(&format!(
                            "{}\t{:.1}\n",
                            strP - count * d,
                            5.0 / 9.0 * ((strP as f64 - count as f64 * d as f64) - 32.0)
                        ));
                    }
                    count += 1;
                } else {
                    break;
                }
            }
        }
    }
    result
}

#[test]
fn test_fahr_to_cel_v() {
    assert_eq!(fahr_to_cel_v(&[]), "Fahr\tCelcius\n");
    assert_eq!(fahr_to_cel_v(&[0, 10, 0]), "Fahr\tCelcius\n0\t-17.8\n");
    assert_eq!(fahr_to_cel_v(&[0, 0, 1]), "Fahr\tCelcius\n0\t-17.8\n");
    assert_eq!(fahr_to_cel_v(&[0,100,10]), "Fahr\tCelcius\n0\t-17.8\n10\t-12.2\n20\t-6.7\n30\t-1.1\n40\t4.4\n50\t10.0\n60\t15.6\n70\t21.1\n80\t26.7\n90\t32.2\n100\t37.8\n");
    assert_eq!(fahr_to_cel_v(&[100,10,10]), "Fahr\tCelcius\n100\t37.8\n90\t32.2\n80\t26.7\n70\t21.1\n60\t15.6\n50\t10.0\n40\t4.4\n30\t-1.1\n20\t-6.7\n10\t-12.2\n");
    assert_eq!(fahr_to_cel_v(&[0,-100,10]), "Fahr\tCelcius\n0\t-17.8\n-10\t-23.3\n-20\t-28.9\n-30\t-34.4\n-40\t-40.0\n-50\t-45.6\n-60\t-51.1\n-70\t-56.7\n-80\t-62.2\n-90\t-67.8\n-100\t-73.3\n");
    assert_eq!(fahr_to_cel_v(&[-100,-10,10]), "Fahr\tCelcius\n-100\t-73.3\n-90\t-67.8\n-80\t-62.2\n-70\t-56.7\n-60\t-51.1\n-50\t-45.6\n-40\t-40.0\n-30\t-34.4\n-20\t-28.9\n-10\t-23.3\n");
}

fn recur_fahr_to_cel_v(v: &[i64]) -> String {
    let mut result = String::new();
    result.push_str(&format!("{}\t{}\n", "Fahr", "Celcius"));
    if v.is_empty() {
    } else {
        let strP: i64 = v[0];
        let endP: i64 = v[1];
        let d: i64 = v[2];
        fn inner_recur_fahr_to_cel_v(
            strP: i64,
            endP: i64,
            d: i64,
            count: i64,
            result: &mut String,
        ) {
            if count != ((strP - endP).abs() / d) + 1 {
                if strP < endP {
                    result.push_str(&format!(
                        "{}\t{:.1}\n",
                        strP + count * d,
                        5.0 / 9.0 * ((strP as f64 + count as f64 * d as f64) - 32.0)
                    ));
                } else {
                    result.push_str(&format!(
                        "{}\t{:.1}\n",
                        strP - count * d,
                        5.0 / 9.0 * ((strP as f64 - count as f64 * d as f64) - 32.0)
                    ));
                }
                inner_recur_fahr_to_cel_v(strP, endP, d, count + 1, result);
            }
        }
        if v[2] == 0 {
            result.push_str(&format!(
                "{}\t{:.1}\n",
                strP + d,
                5.0 / 9.0 * ((strP as f64 + d as f64) - 32.0)
            ));
        } else {
            inner_recur_fahr_to_cel_v(strP, endP, d, 0, &mut result);
        }
    }
    result
}

#[test]
fn test_recur_fahr_to_cel_v() {
    assert_eq!(recur_fahr_to_cel_v(&[]), "Fahr\tCelcius\n");
    assert_eq!(recur_fahr_to_cel_v(&[0, 0, 0]), "Fahr\tCelcius\n0\t-17.8\n");
    assert_eq!(recur_fahr_to_cel_v(&[0, 0, 1]), "Fahr\tCelcius\n0\t-17.8\n");
    assert_eq!(recur_fahr_to_cel_v(&[0,100,10]), "Fahr\tCelcius\n0\t-17.8\n10\t-12.2\n20\t-6.7\n30\t-1.1\n40\t4.4\n50\t10.0\n60\t15.6\n70\t21.1\n80\t26.7\n90\t32.2\n100\t37.8\n");
    assert_eq!(recur_fahr_to_cel_v(&[100,10,10]), "Fahr\tCelcius\n100\t37.8\n90\t32.2\n80\t26.7\n70\t21.1\n60\t15.6\n50\t10.0\n40\t4.4\n30\t-1.1\n20\t-6.7\n10\t-12.2\n");
    assert_eq!(recur_fahr_to_cel_v(&[0,-100,10]), "Fahr\tCelcius\n0\t-17.8\n-10\t-23.3\n-20\t-28.9\n-30\t-34.4\n-40\t-40.0\n-50\t-45.6\n-60\t-51.1\n-70\t-56.7\n-80\t-62.2\n-90\t-67.8\n-100\t-73.3\n");
    assert_eq!(recur_fahr_to_cel_v(&[-100,-10,10]), "Fahr\tCelcius\n-100\t-73.3\n-90\t-67.8\n-80\t-62.2\n-70\t-56.7\n-60\t-51.1\n-50\t-45.6\n-40\t-40.0\n-30\t-34.4\n-20\t-28.9\n-10\t-23.3\n");
}
