use std::fs;

use muninn::*;

#[test]
fn test_get_rtype_preflight() {
    let reqstr = "odin\tpreflight\t/home.odinml".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::PREFLIGHT);
}

#[test]
fn test_get_rtype_pull() {
    let reqstr = "odin\tpull\t/home.odinml".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::PULL);
}

#[test]
fn test_get_rtype_err_wellformed() {
    let reqstr = "odin\terr\t/home.odinml".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::ERR);
}

#[test]
fn test_get_rtype_err_rsplit_len_mismatch() {
    let reqstr = "odin".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::ERR);
}

#[test]
fn test_get_rtype_err_verspec_mismatch() {
    let reqstr = "freyja\tpush\t/home.odinml".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::ERR);
}

#[test]
fn test_get_filepath() {
    let reqstr = "odin\tpull\t/home.odinml".to_string();
    let homedir = "example".to_string();
    let filepath = get_filepath(&reqstr, &homedir);
    assert_eq!(filepath, "example/home.odinml");
}

#[test]
fn test_get_filepath_slash_normalization() {
    let reqstr = "odin\tpull\thome.odinml".to_string();
    let homedir = "example".to_string();
    let filepath = get_filepath(&reqstr, &homedir);
    assert_eq!(filepath, "example/home.odinml");
}

#[test]
fn test_preflight_file_exists() {
    let path = "example/home.odinml".to_string();
    let res = preflight(&path);
    let len = fs::read(path).unwrap().len();
    let comp_res = format!("odin\tA\t{}\r\n", len);
    assert_eq!(res, comp_res);
}

#[test]
fn test_preflight_file_does_not_exist() {
    let path = "example/missing.odinml".to_string();
    let res = preflight(&path);
    assert_eq!(res, "odin\tB\r\n");
}

#[test]
fn test_pull_file_exists() {
    let path = "example/home.odinml".to_string();
    let res = pull(&path);
    let comp_res = fs::read_to_string(path).unwrap();
    assert_eq!(res, comp_res);
}

#[test]
fn test_pull_file_does_not_exist() {
    let path = "example/missing.odinml".to_string();
    let response = pull(&path);
    assert_eq!(response, String::new());
}

#[test]
fn test_error_c() {
    let response = error_c();
    assert_eq!(response, "odin\tC\r\n");
}

