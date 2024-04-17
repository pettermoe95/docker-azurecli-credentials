# Introduction
This is a simple api that fetches an access token through the az logged in user. You can use this to get an access token for your applications that run inside Docker containers,
using host.docker.internal:6436. No need to use service principals and username/password for local Docker development anymore. This saves space as you don't need to install azure cli
on your local docker container, and image size will me smaller.
# Installation
Make sure to have cargo installed to compile the rust application. Then compile the application as a release:
```cargo build --release```
This will compile the application and store the binaries inside a new folder called "target".
Then start the application by running: `./target/release/docker-azurecli-credentials`

You could add the following as aliases in your shell profile script:
```sh
alias entra_at="nohup /path/to/repo/target/release/docker-azurecli-credentials >/dev/null 2>&1 &"
alias entra_at_pid="pgrep -f /path/to/repo/target/release/docker-azurecli-credentials"
```
Then you can use `entra_at` to start the service when you need it, and `entra_at_pid` to find the pid so you can stop it.

# Example code
## Python
``` python
class HostAzCliCredential(TokenCredential):
    def get_token(self, *scopes: str, claims: Optional[str] = None, tenant_id: Optional[str] = None, **kwargs: Any) -> AccessToken:
        resource = _scopes_to_resource(*scopes) # method that converts the scopes to proper resource identifier in the idp
        response = requests.get(f"http://host.docker.internal:6436?resource={resource}")
        if response.ok:
            response_data = response.json()
            access_token = response_data.get("accessToken")
            expires_on = response_data.get("expires_on")
            if access_token:
                return AccessToken(access_token, expires_on)
        raise CredentialUnavailableError(f"Could not get access token from host.docker.internal:6436, resource: {resource}. Make sure the service to get tokens is running.")

    def close(self) -> None:
        pass


class DockerDefaultAzureCredential(DefaultAzureCredential):
    def get_token(self, *scopes: str, claims: Optional[str] = None, tenant_id: Optional[str] = None, **kwargs: Any) -> AccessToken:
        try:
            token = super().get_token(*scopes, claims=claims, tenant_id=tenant_id, **kwargs)
            return token
        except ClientAuthenticationError:
            logger.error("Could not get token from standard DefaultAzureCredential, trying to get access token from host machine...")

        try:
            token = HostAzCliCredential().get_token(*scopes, claims=claims, tenant_id=tenant_id, **kwargs)
            return token
        except CredentialUnavailableError:
            logger.error("Could not get token from host machine, is the entra_at service running?")
        raise ClientAuthenticationError("Completely failed to get credentials")

# This will now try DefaultAzureCredential first and then fall back to the host machines azure cli to get access token
# Like this the deployed application to an App Service will still use ManagedIdentityCredential as usual
credential = DockerDefaultAzureCredential()
key_vault_client = SecretClient("https://my-vault-name.vault.azure.net", credential)
secret = key_vault_client.get_secret("my-secret")
```
# Further work
- Simpler installation
- Example code for how to use it to fetch credentials from within a Docker container
