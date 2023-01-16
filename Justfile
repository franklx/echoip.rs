default:
    @just --list

watch:
    @cargo watch -x run

ARGS := "-o ControlPath=.deploy"
DEST := "root@192.168.128.60"

deploy:
    @cargo build --release
    @-test -S .deploy || ssh {{ARGS}} -M -o ControlPersist=10m {{DEST}} exit
    @ssh {{ARGS}} {{DEST}} systemctl stop echoip
    @scp {{ARGS}} target/release/echoip {{DEST}}:/srv/echoip/
    @ssh {{ARGS}} {{DEST}} systemctl start echoip
    @-test -S .deploy && ssh {{ARGS}} {{DEST}} -O exit
