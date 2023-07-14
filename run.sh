# build & run az-kv-get-secret
# KEYVAULT_URL='https://<vault_name>.vault.azure.net/'
# SECRETS_FILTER='<secret_name_1> <secret_name_2>'
source ../00-ENV/env.sh

# build & run keyvault secret
cd az-kv-get-secret
cargo build

./target/debug/az-kv-get-secret \
    --keyvault-url $KEYVAULT_URL\
    --secrets-filter $SECRETS_FILTER \
    #--only-value