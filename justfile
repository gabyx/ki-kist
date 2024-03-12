set positional-arguments
set shell := ["bash", "-cue"]
root_dir := justfile_directory()
parallel := "true" # Run tasks over components in parallel.
default_regex := ".*"

# Administrative stuff.
###############################################################################
create-cluster *args:
    @cd "{{root_dir}}" && ./tools/create-kind-cluster.sh kikist {{args}}

delete-cluster *args:
    @cd "{{root_dir}}" && ./tools/delete-kind-cluster.sh kikist {{args}}

start-gitlab-runner token *args:
    @cd "{{root_dir}}" && ./tools/start-gitlab-runner.sh {{args}} "{{token}}"

start-db-tool:
    @cd "{{root_dir}}" && dbeaver

# Development stuff.
###############################################################################
start-nix-develop:
    cd {{root_dir}} && nix develop --command zsh

# Deploying the components.
###############################################################################
deploy *args:
    @cd "{{root_dir}}/manifests" && tilt "$@"

deploy-up *args:
    @cd "{{root_dir}}/manifests" && tilt up "$@"

deploy-down *args:
    @cd "{{root_dir}}/manifests" && tilt down "$@"
    # In case anything keeps hanging.
    @kubectl delete all --all --namespace kikist

# Building the components.
###############################################################################
build *args:
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" {{parallel}} "{{default_regex}}" build "${@:1}"

build-selection regex *args:
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" {{parallel}} "{{regex}}" build "${@:2}"

build-image *args:
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" {{parallel}} "{{default_regex}}" build-image "${@:1}"
build-image-selection regex *args:
    cd {{root_dir}} && \
        "{{root_dir}}/tools/run-components-parallel.sh" {{parallel}} "{{regex}}" build-image "${@:2}"

watch:
    cd "{{root_dir}}" && cargo watch -x 'build'

# Component functionality.
###############################################################################
component component task *args:
    @echo "Component '{{component}}': task: {{task}}" && \
    echo -e "| =========================================" && \
    "{{root_dir}}/tools/run-component-task.sh" "{{component}}" "{{task}}" "${@:3}" 2>&1 && \
    echo -e "| =========================================\n\n"

list-components regex=".*":
    @cd "{{root_dir}}" && find ./components -mindepth 1 -maxdepth 1 \
        -type d -regextype "posix-extended" -regex "./components/{{regex}}"

# Testing functionality.
###############################################################################
test what="manual":
    @cd "{{root_dir}}/tests/{{what}}" && \
        just run

client *args:
    @cd components/client && \
        just run --cwd "{{root_dir}}" {{args}}

# Formatting.
###############################################################################
format regex=".*":
    cd "{{root_dir}}" && \
       tools/format.sh "{{parallel}}" "{{regex}}"

# Linting.
###############################################################################
lint regex=".*":
    cd "{{root_dir}}" && \
        tools/lint.sh "{{parallel}}" "{{regex}}"

lint-docs regex=".*":
    cd "{{root_dir}}/docs" && \
        just lint

# CI Stuff
###############################################################################
upload-ci-images:
    cd "{{root_dir}}" && \
        .gitlab/scripts/upload-images.sh
