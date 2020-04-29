set -ex

main () {
    local src=$(pwd) \
    stage= \
    bin_name=
    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            bin_name=$CRATE_NAME
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            bin_name=$CRATE_NAME
            ;;
        windows)
            stage=$(mktemp -d -t tmp)
            bin_name=$CRATE_NAME.exe
            ;;
    esac
    cargo build --release

    cp target/release/$CRATE_NAME $stage/
    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TRAVIS_OS_NAME.tar.gz $bin_name
    cd $src
    rm -rf $stage
}

main