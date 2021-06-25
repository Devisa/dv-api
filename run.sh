#!/usr/bin/env bash
# vi: ft=bash

_GREEN=$(tput setaf 2)
_BLUE=$(tput setaf 4)
_MAG=$(tput setaf 5)
_RED=$(tput setaf 1)
_RESET=$(tput sgr0)
_BOLD=$(tput bold)

PORT=1888
HOST="0.0.0.0"

push_container_img() {
    docker tag local-image:tagname new-repo:tagname
    docker push new-repo:tagname
}




export PS1="${_GREEN}\h${_BLUE}@${_RED}\u${_RESET} ${_BOLD}\$ ${_RESET}"
version="0.0.1"
echo "===================================================================="
echo "| Welcome to the api.devisa v0.0.1 build & run tool! (4/25/2021)    "
echo "|     1) [dvv] build  -   Build a container image of api.devisa     "
echo "|     2) [dvv] run    -   Run a container image of api.devisa       "
echo "|     3) [dvv] test   -   Perform staging/testing                   "
echo "|     4) [dvv] deploy -   Deploy api.devisa to production           "
echo "|     ?) [dvv] help   -   Get help on functions and controls        "
echo "===================================================================="
echo "                                                                     "

function output_test_help() {
    echo "       DEVISA BUILD & RUN SCRIPT HELP "
    echo "             (version $version)              "
    echo "                                             "
    echo "- DEPLOY SUBCOMMAND FLAGS                    "
    echo "  Non-positional                             "
    echo "                                             "
    echo "(-h | --help)                           Print out this help message"
    echo "                                             "
    echo "- DEPLOY SUBCOMMAND ARGS                     "
    echo "  Positional                                 "
    echo "                                             "
}
function output_deploy_help() {
    echo "       DEVISA BUILD & RUN SCRIPT HELP "
    echo "             (version $version)              "
    echo "                                             "
    echo "- DEPLOY SUBCOMMAND FLAGS                    "
    echo "  Non-positional                             "
    echo "                                             "
    echo "(-h | --help)                           Print out this help message"
    echo "                                             "
    echo "- DEPLOY SUBCOMMAND ARGS                     "
    echo "  Positional                                 "
    echo "                                             "
}
function output_run_help() {
    echo "       DEVISA BUILD & RUN SCRIPT HELP "
    echo "             (version $version)              "
    echo "                                             "
    echo "- RUN SUBCOMMAND FLAGS                       "
    echo "  Non-positional                             "
    echo "                                             "
    echo "(-v | --verbose)                        Specify console output to be verbose"
    echo "(-h | --help)                           Print out this help message"
    echo "                                             "
    echo "- RUN SUBCOMMAND ARGS                        "
    echo "  Positional                                 "
    echo "                                             "
    echo "(-f | --file) <DOCKERFILE>              Manually specify the dockerfile location"
    echo "(-p | --port) <PORT>                    Specify the IPv4 address port to run on"
    echo "(-a | --address) <ADDRESS>              Specify thhe IPv4 address to run on"
}
function output_build_help() {
    echo "       DEVISA BUILD & RUN SCRIPT HELP "
    echo "             (version $version)              "
    echo "                                             "
    echo "- ${_GREEN}BUILD SUBCOMMAND FLAGS${_RESET}                     "
    echo "  Non-positional                             "
    echo "                                             "
    echo "(-h | --help)                           Print out this help message"
    echo "(-f | --file)                           Manually specify dockerfile location"
    echo "(-v | --verbose)                        Specify console output to be verbose"
    echo "(-l | --log)                            Log running/debugging output"
    echo "                                             "
    echo "- ${_GREEN}BUILD SUBCOMMAND ARGS${_RESET}                      "
    echo "  Positional                                 "
    echo "                                             "
    echo "(-f | --file) <DOCKERFILE>              Manually specify the dockerfile location"
    echo "(-p | --port) <PORT>                           Manually specify dockerfile location"
    echo "(-v | --verbose)                        Specify console output to be verbose"
    echo "(-l | --log)                            Log running/debugging output"
}
function api_build() {
    echo "BUILD"
}

subcommand=$1
shift
case $subcommand in
    build)
        echo "${_GREEN}${_BOLD}====  BUILD  ========================================================${_RESET}"
        stage="dev"
        verbose='false'
        run='false'
        dockerfile="./Dockerfile"
        tag="dvapi"
        while getopts ab:e:f:dDp:a:hrt:vV: o; do
            case $o in
                p)  stage="prod"
                    echo "|     - Production set to ${_GREEN}true${_RESET}     "
                    ;;
                e)
                    echo "|     - Environmental variable $o set to ${_GREEN}$OPTARG${_RESET}     "
                    ;;
                f)
                    dockerfile=$OPTARG
                    echo "|     - Dockerfile set to ${_GREEN}${_BOLD}$dockerfile${_RESET}                               "
                    ;;
                r)
                    run='true'
                    echo "|     - Container will be run after being built       "
                    ;;
                t)
                    tag=$OPTARG
                    echo "|     - Container tag  set to ${_GREEN}${_BOLD}$tag${_RESET}               "
                    ;;
                v)  verbose='true'
                    echo "|     - Verbose console output set to ${_BOLD}true${_RESET}        "
                    ;;
                V)  vers=$OPTARG
                    echo "|     - Docker container image version set to ${_GREEN}$vers{_RESET}   "
                    ;;
                h) output_build_help ;;
            esac
        done
        echo "${_GREEN}${_BOLD}====  BEGINNING BUILD...  ===========================================${_RESET}"
        podman build -t $tag -f ./Dockerfile.dev

        echo "${_GREEN}${_BOLD}====  RUNNING BUILD...  =============================================${_RESET}"
        podman docker run -ti --rm $tag

        ;;
    run)
        echo "${_RED}====  RUN  ==========================================================${_RESET}"
        while getopts abf:dDp:a:h o; do
            case $o in
                a) echo "a was trigg param $OPTARG" >&2 ;;
                b) echo "b was d $OPTARG" >&2 ;;
                d) echo "deploy" ;;
                h) output_run_help ;;
            esac
        done
        ;;
    deploy)
        echo "${BLUE}====  DEPLOY  =======================================================${_RESET}"
        while getopts abf:dDp:a:h o; do
            case $o in
                a) echo "a was trigg param $OPTARG" >&2 ;;
                b) echo "b was d $OPTARG" >&2 ;;
                d) echo "deploy" ;;
                h) output_deploy_help ;;
            esac
        done
        ;;
    test)
        echo "${MAG}====  TEST  =========================================================${_RESET}"
        while getopts abf:dDp:a:h o; do
            case $o in
                a) echo "a was trigg param $OPTARG" >&2 ;;
                b) echo "b was d $OPTARG" >&2 ;;
                d) echo "deploy" ;;
                h) output_test_help ;;
            esac
        done
        ;;
esac



#dockerfile='./Dockerfile'
#debug='false'
#verbose='false'
#build='false'
#run='false'
#dev='prod'
#bflag=''
#files=''
#port=1888


