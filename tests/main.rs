use std::fs;
use rydja_server::*;

#[test]
fn test_get_rtype_pull() {
    let reqstr = "rydja1\tpull\t/home.ryd".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::PULL);
}

#[test]
fn test_get_rtype_push() {
    let reqstr = "rydja1\tpush\t/home.ryd".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::PUSH);
}

#[test]
fn test_get_rtype_err_wellformed() {
    let reqstr = "rydja1\terr\t/home.ryd".to_string();
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
    let reqstr = "rydja\tpush\t/home.ryd".to_string();
    let rtype = get_rtype(&reqstr);
    assert_eq!(rtype, ReqType::ERR);
}

#[test]
fn test_get_filepath() {
    let reqstr = "rydja1\tpull\t/home.ryd".to_string();
    let homedir = "example".to_string();
    let filepath = get_filepath(&reqstr, &homedir);
    assert_eq!(filepath, "example/home.ryd");
}

#[test]
fn test_get_filepath_slash_normalization() {
    let reqstr = "rydja1\tpull\thome.ryd".to_string();
    let homedir = "example".to_string();
    let filepath = get_filepath(&reqstr, &homedir);
    assert_eq!(filepath, "example/home.ryd");
}

#[test]
fn test_pull_file_exists() {
    let path = "example/home.ryd".to_string();
    let response = pull(&path);
    let content = fs::read_to_string(path).unwrap();
    let comp_response = format!("rydja1\tA\r\n{}", content);
    assert_eq!(response, comp_response);
}

#[test]
fn test_pull_file_does_not_exist() {
    let path = "example/missing.ryd".to_string();
    let response = pull(&path);
    assert_eq!(response, "rydja1\tB\r\nFile not found");
}

#[test]
fn test_push() {
    let response = push();
    assert_eq!(response, "rydja1\tC\r\nRequest method \'push\' unsupported");
}

#[test]
fn test_error_c() {
    let response = error_c(ReqType::ERR);
    assert_eq!(response, "rydja1\tC\r\n");
}

#[test]
fn test_error_c_push() {
    let response = error_c(ReqType::PUSH);
    assert_eq!(response, "rydja1\tC\r\nRequest method \'push\' unsupported");
}

#[test]
fn test_error_d() {
    let response = error_d();
    assert_eq!(response, "rydja1\tD\r\nMalformed request");
}

