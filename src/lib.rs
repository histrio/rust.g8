#![crate_id = "projectname"]
#![crate_type = "lib"]

#[cfg(test)]
mod test {

  #[test]
  fn should_pass() {
    assert_eq!(1, 3);
  }
}
