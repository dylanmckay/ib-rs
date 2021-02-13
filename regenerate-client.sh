#! /bin/sh -ea

SCRIPT_DIR=$(realpath $(dirname $0))
cd "${SCRIPT_DIR}"

echo "Downliading the IB 'swagger.json' file"
curl https://interactivebrokers.github.io/cpwebapi/swagger.json -o etc/swagger.json

if which json_pp &> /dev/null; then
  echo "note: json_pp is installed. formatting 'swagger.json'"
  cat ./etc/swagger.json | json_pp > ./etc/swagger.formatted.json
  mv ./etc/swagger.formatted.json ./etc/swagger.json
else
  echo "warning: json_pp is NOT installed. the 'swagger.json' file will not be formatted"
fi

echo "Applying Rust-swagger-specific fixups to the 'swagger.json' file as 'swagger.fixed.json'"

echo "  -> all Rust fields are snake cased, but the 'swagger.json' file has a model with both 'companyName' and 'company_name', causing conflicts"
jq 'walk(if type == "object" and has("companyName") and has("company_name") then del(.companyName) else . end)' ./etc/swagger.json > ./etc/swagger.fixed.json

echo "Moving 'swagger.fixed.json' over top of 'swagger.json'"
mv ./etc/swagger.fixed.json ./etc/swagger.json

echo "Pulling the swagger-codegen-cli docker image"
docker pull swaggerapi/swagger-codegen-cli

output_staging_dir=$(mktemp -qd)
output_staging_dir_final=$(mktemp -qd)

echo "output staging directory is '${output_staging_dir}'"

docker run \
  -v $(realpath ./etc):/ib-etc \
  -v ${output_staging_dir}:/ib-gen \
  swaggerapi/swagger-codegen-cli generate \
  -i /ib-etc/swagger.json \
  -l rust \
  -o /ib-gen

echo "copying staging directory '${output_staging_dir}' to '${output_staging_dir_final}' to fix docker permissions"
cp -r --no-preserve=mode ${output_staging_dir}/* "${output_staging_dir_final}"


echo "modifying the generated Rust crate sources from the defaults"
cd "${output_staging_dir_final}"

echo "  -> updating Cargo.toml"
sed -i 's/name = "swagger"/name = "ib"/g' Cargo.toml && sed -i 's/authors =.*$/authors = ["Dylan McKay <me@dylanmckay.io>"]/g' Cargo.toml

echo "  -> deleting junk files"
rm git_push.sh

echo "  -> adding Rust imports that are missing"
find . -name "*.rs" -exec sed -i 's/use serde_json;/use serde_json::{self, Value};/g' {} \;

cd "${SCRIPT_DIR}"

echo "removing preexisting files prior to overwrite to avoid dead/redundant/orphaned files"
rm -rf ./src ./docs

echo "moving staging directory into version control"
cp -r ${output_staging_dir_final}/* ./

