RC = rustc
OUT_DIR = ./build

OPT = -lldap
DEBUG_OPT = -g
RELEASE_OPT = -O
TEST_OPT = --test

CrateName = ldap
CrateType = rlib

all: debug

prelude:
	mkdir -p ${OUT_DIR}

# Build a release candidate
release:
	mkdir -p ${OUT_DIR}/release/
	${RC} --crate-name ${CrateName} --crate-type ${CrateType} ${OPT} ${RELEASE_OPT} --out-dir ${OUT_DIR}/release/ src/lib.rs

# Build a debug candidate
debug:
	mkdir -p ${OUT_DIR}/debug/
	${RC} --crate-name ${CrateName} --crate-type ${CrateType} ${OPT} ${DEBUG_OPT} --out-dir ${OUT_DIR}/debug/ src/lib.rs

test: debug
	mkdir -p ${OUT_DIR}/test/
	${RC} --crate-name ${CrateName} --crate-type ${CrateType} ${OPT} ${TEST_OPT} --out-dir ${OUT_DIR}/test/ src/lib.rs
	${OUT_DIR}/test/${CrateName}
	${RC} --crate-name ${CrateName}-test --crate-type bin ${DEBUG_OPT} -L./build/debug --out-dir ${OUT_DIR}/test/ src/main.rs
	${OUT_DIR}/test/${CrateName}-test

clean:
	rm -rf ${OUT_DIR}
