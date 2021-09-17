use std::fs;

use muninn::*;

#[test]
fn test_get_rtype_pull() {
    let reqstr = "odin\tpull\t/home.odinml".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::PULL);
}

#[test]
fn test_get_rtype_push() {
    let reqstr = "odin\tpush\t/home.odinml".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::PUSH);
}

#[test]
fn test_get_rtype_err_wellformed() {
    let reqstr = "odin\terr\t/home.odinml".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::ERR);
}

#[test]
fn test_get_rtype_err_rsplit_len_mismatch() {
    let reqstr = "malformed request".to_string();
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
fn test_pull_file_exists() {
    let path = "example/home.odinml".to_string();
    let response = pull(&path);
    let content = fs::read_to_string(path).unwrap();
    let comp_response = format!("odin\tA\r\n{}", content);
    assert_eq!(response, comp_response);
}

#[test]
fn test_pull_file_does_not_exist() {
    let path = "example/missing.odinml".to_string();
    let response = pull(&path);
    assert_eq!(response, "odin\tB\r\nFile not found");
}

#[test]
fn test_push() {
    let response = push();
    assert_eq!(response, "odin\tC\r\nRequest method \'push\' unsupported");
}

#[test]
fn test_error_c() {
    let response = error_c(ReqType::ERR);
    assert_eq!(response, "odin\tC\r\n");
}

#[test]
fn test_error_c_push() {
    let response = error_c(ReqType::PUSH);
    assert_eq!(response, "odin\tC\r\nRequest method \'push\' unsupported");
}

#[test]
fn test_error_d() {
    let response = error_d();
    assert_eq!(response, "odin\tD\r\nMalformed request");
}

