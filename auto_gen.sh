# Generates all the necessary mysql bindings for you.
# Needs bindgen from https://github.com/crabtw/rust-bindgen placed in the base directory.

function gen() {
	./bindgen -l mysqlclient -o ./src/ffi/"$1".rs /usr/include/mysql/"$1".h
	echo "pub mod $1;" >> ./src/ffi/mod.rs
}

rm ./src/ffi/mod.rs
touch ./src/ffi/mod.rs
echo "#![allow(dead_code, non_camel_case_types)]" >> ./src/ffi/mod.rs

gen mysql

