group "default" {
  targets = ["csv", "csv_again"]
}

target "common" {
  context   = "."
  dockerfile = "containers/rust_binary/Dockerfile"
}

target "csv" {
  inherits = ["common"]
  tags = ["csv:latest"]
  args = { BIN = "csv" }
}

target "csv_again" {
  inherits = ["common"]
  tags = ["csv_again:latest"]
  args = { BIN = "csv_again" }
}
