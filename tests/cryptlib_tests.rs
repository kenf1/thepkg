#[cfg(test)]

#[cfg(feature = "crypt")]
use thepkg::cryptlib;

//encrypt pass
#[cfg(feature = "crypt")]
#[test]
fn encrypt_test_pass(){
    let dict = String::from("abcde");
    let entry = String::from("abc");

    let res = cryptlib::cryptfn("encrypt",&entry,&dict,1);
    assert!(matches!(res,Ok(ref s) if s == "bcd"));
}

//encrypt fail: char not found
#[cfg(feature = "crypt")]
#[test]
fn encrypt_test_fail(){
    let dict = String::from("abcde");
    let entry = String::from("thepkg");

    let res = cryptlib::cryptfn("encrypt",&entry,&dict,1);
    assert_eq!(
        res,
        Err(String::from("Error: entered string contains characters not found in reference dict."))
    );
}

//decrypt pass
#[cfg(feature = "crypt")]
#[test]
fn decrypt_test_pass() {
    let dict = String::from("abcde");
    let entry = String::from("bcd");

    let res = cryptlib::cryptfn("decrypt", &entry, &dict, 1);
    assert!(matches!(res, Ok(ref s) if s == "abc"));
}

#[cfg(feature = "crypt")]
#[test]
fn decrypt_test_fail() {
    let dict = String::from("abcde");
    let entry = String::from("b1cd");

    let res = cryptlib::cryptfn("decrypt",&entry,&dict,1);
    assert_eq!(
        res,
        Err(String::from("Error: entered string contains characters not found in reference dict."))
    );
}