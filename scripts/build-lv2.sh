PACKAGE_NAME=(`./scripts/get-package-name.sh lv2`)
NAME=$(echo $PACKAGE_NAME | perl -pe 's/dm_+([^\W_])/dm-\U$1/g' | perl -pe 's/(?<=[^\W_])_+([^\W_])/\U$1/g')
LV2_NAME="$NAME.so"
BINARY_NAME="lib$PACKAGE_NAME.so"
MOVE_FROM="target/release/$BINARY_NAME"
MOVE_TO="target/release/$LV2_NAME"

cd lv2
cargo build --release

if [ -d "$MOVE_TO" ]; then
  rm -r "$MOVE_TO"
fi

if mv "$MOVE_FROM" "$MOVE_TO"; then
  echo "Finished compiling LV2 plugin. File can be found here: '$MOVE_TO'."
fi