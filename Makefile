PROTO_SRC_DIR=proto_files
PROTO_RS_DST_DIR=src
PROTO_PY_DST_DIR=client

build:
	cargo build

compile_protobuf:
	cargo run --bin gen -- --out_dir ${PROTO_RS_DST_DIR} --input ${PROTO_SRC_DIR}/test.proto --include ${PROTO_SRC_DIR}
	protoc -I=${PROTO_SRC_DIR} \
        --python_out="${PROTO_PY_DST_DIR}" \
		${PROTO_SRC_DIR}/test.proto

_compile_protobuf:
	protoc -I=${PROTO_SRC_DIR} \
        --rust_out="${PROTO_RS_DST_DIR}" \
		${PROTO_SRC_DIR}/test_with_rust_ext.proto

run:
	RUST_BACKTRACE=1 ENV=dev cargo run --bin server -- --port 18081 --web_workers 1

build_docker:
	docker build -t actix-web-x-protobuf-example .

build_debug_docker:
	docker build -t actix-web-x-protobuf-example-debug --build-arg BUILD_MODE=debug .

run_docker:
	docker run -it -p 18080:18080 \
		--ulimit nofile=262144:262144 \
	    -e ENV=dev \
	    -e RUST_LOG_STYLE=always \
        -e RUST_LOG="server=info,actix_web=info" \
	    actix-web-x-protobuf-example \
	    --port 18080 \
	    --web_workers 1

run_docker_with_backtrace:
	docker run -it -p 18080:18080 \
		--ulimit nofile=262144:262144 \
	    -e RUST_BACKTRACE=1 \
	    -e ENV=dev \
	    -e RUST_LOG_STYLE=always \
        -e RUST_LOG="debug,actix_web=debug" \
	    actix-web-x-protobuf-example-debug \
	    --port 18080 \
	    --web_workers 1
